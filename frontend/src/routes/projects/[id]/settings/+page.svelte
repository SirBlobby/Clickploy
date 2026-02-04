<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import {
		getProject,
		type Project,
		updateProjectEnv,
		updateProject,
		getSystemStatus,
	} from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import {
		Select,
		SelectContent,
		SelectItem,
		SelectTrigger,
	} from "$lib/components/ui/select";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
		CardFooter,
	} from "$lib/components/ui/card";
	import {
		Loader2,
		Save,
		Trash2,
		Eye,
		EyeOff,
		Plus,
		Copy,
		Upload,
	} from "@lucide/svelte";
	import { toast } from "svelte-sonner";

	let project = $state<Project | null>(null);
	let loading = $state(true);
	let showSecret = $state(false);
	let systemStatus = $state<{ local_ip: string; public_ip: string } | null>(
		null,
	);

	let tempEnvVars = $state<{ key: string; value: string }[]>([]);
	let isDirty = $state(false);
	let formLoading = $state(false);

	let projectName = $state("");
	let repoUrl = $state("");
	let gitToken = $state("");
	let runtime = $state("");
	let installCmd = $state("");
	let buildCmd = $state("");
	let startCmd = $state("");

	const runtimeConfig: Record<
		string,
		{ install: string; build: string; start: string }
	> = {
		nodejs: {
			install: "npm install",
			build: "npm run build",
			start: "npm start",
		},
		bun: {
			install: "bun install",
			build: "bun run build",
			start: "bun start",
		},
		python: {
			install: "pip install -r requirements.txt",
			build: "",
			start: "python3 main.py",
		},
		go: {
			install: "go mod download",
			build: "go build -o main .",
			start: "./main",
		},
		rust: {
			install: "cargo fetch",
			build: "cargo build --release",
			start: "./target/release/main",
		},
		php: {
			install: "composer install",
			build: "",
			start: "php -S 0.0.0.0:8080",
		},
		java: {
			install: "mvn clean install",
			build: "mvn package",
			start: "java -jar target/app.jar",
		},
		static: {
			install: "",
			build: "",
			start: "",
		},
		dockerfile: {
			install: "",
			build: "",
			start: "",
		},
	};

	let defaults = $derived(
		runtimeConfig[runtime] || {
			install: "npm install",
			build: "npm run build",
			start: "npm start",
		},
	);

	onMount(async () => {
		const id = $page.params.id;
		const [projRes, sysRes] = await Promise.all([
			id ? getProject(id) : null,
			getSystemStatus(),
		]);

		if (projRes) {
			project = projRes;
			initFormData();
			initEnvVars();
		}

		if (sysRes) {
			systemStatus = sysRes;
		}
		loading = false;
	});

	function initFormData() {
		if (!project) return;
		projectName = project.name;
		repoUrl = project.repo_url;
		runtime = project.runtime || "nodejs";
		installCmd = project.install_command || "";
		buildCmd = project.build_command || "";
		startCmd = project.start_command || "";
	}

	async function saveSettings() {
		if (!project) return;
		formLoading = true;
		const res = await updateProject(project.id, {
			name: projectName,
			repo_url: repoUrl,
			runtime,
			install_command: installCmd,
			build_command: buildCmd,
			start_command: startCmd,
			...(gitToken ? { git_token: gitToken } : {}),
		});
		formLoading = false;

		if (res) {
			project = res;
			initFormData();
			toast.success("Settings updated successfully");
		}
	}

	function copyId() {
		if (project) {
			navigator.clipboard.writeText(project.id);
			toast.success("Project ID copied");
		}
	}

	function initEnvVars() {
		if (project?.env_vars) {
			tempEnvVars = project.env_vars.map((e) => ({
				key: e.key,
				value: e.value,
			}));
		} else {
			tempEnvVars = [];
		}
		isDirty = false;
	}

	function toggleSecret() {
		showSecret = !showSecret;
	}

	function addEnvVar() {
		tempEnvVars = [...tempEnvVars, { key: "", value: "" }];
		isDirty = true;
	}

	function removeEnvVar(index: number) {
		tempEnvVars = tempEnvVars.filter((_, i) => i !== index);
		isDirty = true;
	}

	function handleKeyInput(index: number, e: Event) {
		const target = e.target as HTMLInputElement;
		tempEnvVars[index].key = target.value;
		isDirty = true;
	}

	function handleValueInput(index: number, e: Event) {
		const target = e.target as HTMLInputElement;
		tempEnvVars[index].value = target.value;
		isDirty = true;
	}

	async function saveEnvVars() {
		if (!project) return;

		const envMap: Record<string, string> = {};
		for (const e of tempEnvVars) {
			if (e.key.trim()) envMap[e.key.trim()] = e.value;
		}

		loading = true;
		const success = await updateProjectEnv(project.id.toString(), envMap);
		loading = false;

		if (success) {
			toast.success("Environment variables updated successfully");
			const res = await getProject(project.id.toString());
			if (res) {
				project = res;
				initEnvVars();
			}
		} else {
			toast.error("Failed to update environment variables");
		}
	}

	function copyWebhook() {
		if (!project) return;
		const displayUrl = `http://localhost:8080/projects/${project.id}/webhook/${project.webhook_secret}`;
		navigator.clipboard.writeText(displayUrl);
		toast.success("Webhook URL copied");
	}

	async function handleFileUpload(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file) return;

		const text = await file.text();
		const lines = text.split("\n");

		for (const line of lines) {
			const trimmed = line.trim();
			if (!trimmed || trimmed.startsWith("#")) continue;

			const [key, ...parts] = trimmed.split("=");
			if (key) {
				const value = parts.join("=");
				const existingIndex = tempEnvVars.findIndex(
					(e) => e.key === key.trim(),
				);
				if (existingIndex >= 0) {
					tempEnvVars[existingIndex].value = value.replace(
						/^["'](.*)["']$/,
						"$1",
					); // remove quotes
				} else {
					tempEnvVars = [
						...tempEnvVars,
						{
							key: key.trim(),
							value: value.replace(/^["'](.*)["']$/, "$1"),
						},
					];
				}
			}
		}
		isDirty = true;
		target.value = "";
		toast.success("Parsed .env file successfully");
	}
</script>

{#if loading && !project}
	<div class="flex justify-center p-10">
		<Loader2 class="h-8 w-8 animate-spin" />
	</div>
{:else if project}
	<div class="space-y-6 max-w-4xl">
		<Card class="border-border/60">
			<CardHeader>
				<CardTitle class="text-xl font-bold"
					>Git Configuration</CardTitle
				>
				<CardDescription>
					Connect your project to a Git repository.
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-2">
					<Label for="name">Project Name</Label>
					<Input
						id="name"
						bind:value={projectName}
						placeholder="my-awesome-project"
					/>
				</div>
				<div class="grid gap-2">
					<Label for="repo">Repository URL</Label>
					<Input
						id="repo"
						bind:value={repoUrl}
						placeholder="https://github.com/user/repo"
					/>
				</div>
				<div class="grid gap-2">
					<Label for="token">Git Token (Optional)</Label>
					<Input
						id="token"
						type="password"
						bind:value={gitToken}
						placeholder="Start typing to update..."
					/>
					<p class="text-[0.8rem] text-muted-foreground">
						Leave empty to keep the existing token.
					</p>
				</div>
			</CardContent>
			<CardFooter class="border-t px-4 flex justify-end">
				<Button onclick={saveSettings} disabled={formLoading} size="sm">
					{#if formLoading}
						<Loader2 class="h-4 w-4 mr-2 animate-spin" /> Saving...
					{:else}
						<Save class="h-4 w-4 mr-2" /> Save Changes
					{/if}
				</Button>
			</CardFooter>
		</Card>

		<Card class="border-border/60">
			<CardHeader>
				<CardTitle class="text-xl font-bold"
					>Build & Output Settings</CardTitle
				>
				<CardDescription>
					Configure how your application is built and run.
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-2">
					<Label for="runtime">Runtime</Label>
					<Select type="single" bind:value={runtime}>
						<SelectTrigger class="w-full">
							{runtime
								? runtime.charAt(0).toUpperCase() +
									runtime.slice(1)
								: "Select a runtime"}
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="nodejs">Node.js</SelectItem>
							<SelectItem value="bun">Bun</SelectItem>
							<SelectItem value="python">Python</SelectItem>
							<SelectItem value="go">Go</SelectItem>
							<SelectItem value="rust">Rust</SelectItem>
							<SelectItem value="php">PHP</SelectItem>
							<SelectItem value="java">Java</SelectItem>
							<SelectItem value="static">Static</SelectItem>
							<SelectItem value="dockerfile"
								>Dockerfile</SelectItem
							>
						</SelectContent>
					</Select>
				</div>
				<div class="grid gap-2">
					<Label for="install">Install Command</Label>
					<Input
						id="install"
						bind:value={installCmd}
						placeholder={defaults.install}
					/>
				</div>
				<div class="grid gap-2">
					<Label for="build">Build Command</Label>
					<Input
						id="build"
						bind:value={buildCmd}
						placeholder={defaults.build}
					/>
				</div>
				<div class="grid gap-2">
					<Label for="start">Start Command</Label>
					<Input
						id="start"
						bind:value={startCmd}
						placeholder={defaults.start}
					/>
				</div>
			</CardContent>
			<CardFooter class="border-t px-4 flex justify-end">
				<Button onclick={saveSettings} disabled={formLoading} size="sm">
					{#if formLoading}
						<Loader2 class="h-4 w-4 mr-2 animate-spin" /> Saving...
					{:else}
						<Save class="h-4 w-4 mr-2" /> Save Changes
					{/if}
				</Button>
			</CardFooter>
		</Card>

		<Card class="border-border/60">
			<CardHeader>
				<CardTitle class="text-xl font-bold">Networking</CardTitle>
				<CardDescription>
					Manage network settings for your deployment.
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-2">
					<Label for="port">Internal Port</Label>
					<Input
						id="port"
						value={project.port.toString()}
						readonly
						class="bg-muted font-mono"
					/>
					<p class="text-[0.8rem] text-muted-foreground">
						This port is assigned by the system and cannot be
						changed.
					</p>
				</div>
				{#if systemStatus}
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
						<div class="grid gap-2">
							<Label>Local Network URL</Label>
							<div class="flex items-center gap-2">
								<Input
									readonly
									value={`http://${systemStatus.local_ip}:${project.port}`}
									class="bg-muted font-mono"
								/>
								<Button
									variant="outline"
									size="icon"
									onclick={() =>
										window.open(
											`http://${systemStatus!.local_ip}:${project!.port}`,
											"_blank",
										)}
								>
									<Eye class="h-4 w-4" />
								</Button>
							</div>
						</div>
						<div class="grid gap-2">
							<Label>Public Network URL</Label>
							<div class="flex items-center gap-2">
								<Input
									readonly
									value={`http://${systemStatus.public_ip}:${project.port}`}
									class="bg-muted font-mono"
								/>
								<Button
									variant="outline"
									size="icon"
									onclick={() =>
										window.open(
											`http://${systemStatus!.public_ip}:${project!.port}`,
											"_blank",
										)}
								>
									<Eye class="h-4 w-4" />
								</Button>
							</div>
						</div>
					</div>
				{/if}
			</CardContent>
		</Card>

		<Card class="border-border/60">
			<CardHeader class="pb-4">
				<div class="flex items-center justify-between">
					<div>
						<CardTitle class="text-xl font-bold"
							>Environment Variables</CardTitle
						>
						<CardDescription
							class="mt-1 text-sm text-muted-foreground"
						>
							Configure runtime environment variables.
						</CardDescription>
					</div>
					<Button
						variant="ghost"
						size="sm"
						onclick={toggleSecret}
						class="h-8"
					>
						{#if showSecret}
							<EyeOff class="h-4 w-4" />
						{:else}
							<Eye class="h-4 w-4" />
						{/if}
					</Button>
				</div>
			</CardHeader>
			<CardContent class="space-y-3 pb-4">
				{#each tempEnvVars as env, i}
					<div class="flex gap-3 items-center">
						<Input
							placeholder="Key"
							value={env.key}
							oninput={(e) => handleKeyInput(i, e)}
							class="flex-1 h-11 bg-background/50 border-border/60 font-mono"
						/>
						<Input
							placeholder="Value"
							value={env.value}
							type={showSecret ? "text" : "password"}
							oninput={(e) => handleValueInput(i, e)}
							class="flex-1 h-11 bg-background/50 border-border/60"
						/>
						<Button
							variant="ghost"
							size="icon"
							class="h-11 w-11 shrink-0"
							onclick={() => removeEnvVar(i)}
						>
							<Trash2 class="h-4 w-4" />
						</Button>
					</div>
				{/each}

				<div class="flex gap-2">
					<Button
						variant="outline"
						class="flex-1 h-11 border-dashed border-border/60 hover:bg-muted/50"
						onclick={addEnvVar}
					>
						<Plus class="h-4 w-4 mr-2" />
						Add Variable
					</Button>
					<div class="relative">
						<input
							type="file"
							accept=".env"
							class="hidden"
							id="env-upload"
							onchange={handleFileUpload}
						/>
						<Label
							for="env-upload"
							class="flex items-center justify-center px-4 h-11 rounded-md border border-dashed border-border/60 hover:bg-muted/50 cursor-pointer bg-background"
						>
							<Upload class="h-4 w-4 mr-2" />
							Upload .env
						</Label>
					</div>
				</div>
			</CardContent>
			<CardFooter class="border-t px-4 flex justify-end">
				<Button
					onclick={saveEnvVars}
					disabled={!isDirty || loading}
					size="sm"
				>
					{#if loading}
						<Loader2 class="h-4 w-4 mr-2 animate-spin" /> Saving...
					{:else}
						<Save class="h-4 w-4 mr-2" /> Save Changes
					{/if}
				</Button>
			</CardFooter>
		</Card>

		<Card class="border-border/60">
			<CardHeader>
				<CardTitle class="text-lg">Webhook Integration</CardTitle>
				<CardDescription class="mt-1.5">
					Trigger deployments automatically when you push to your
					repository.
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label class="text-sm">Webhook URL</Label>
					<div class="flex items-center gap-2">
						<Input
							readonly
							value={`http://localhost:8080/projects/${project.id}/webhook/${project.webhook_secret}`}
							class="bg-muted font-mono text-xs"
						/>
						<Button
							variant="outline"
							size="icon"
							onclick={copyWebhook}
						>
							<Copy class="h-4 w-4" />
						</Button>
					</div>
				</div>
			</CardContent>
		</Card>

		<Card class="border-destructive/50 bg-destructive/5">
			<CardHeader>
				<CardTitle class="text-destructive text-lg"
					>Danger Zone</CardTitle
				>
			</CardHeader>
			<CardContent>
				<div class="flex items-center justify-between">
					<div>
						<h4 class="font-medium">Delete Project</h4>
						<p class="text-sm text-muted-foreground">
							This action cannot be undone.
						</p>
					</div>
					<Button variant="destructive">Delete Project</Button>
				</div>
			</CardContent>
		</Card>
	</div>
{/if}
