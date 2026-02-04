<script lang="ts">
	import { user, logout } from "$lib/auth";
	import { listProjects, createProject, type Project } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
	} from "$lib/components/ui/card";
	import * as Dialog from "$lib/components/ui/dialog";
	import {
		Loader2,
		Plus,
		Github,
		LogOut,
		Terminal,
		Activity,
		Settings,
		ChevronsUpDown,
		ExternalLink,
		Upload,
	} from "@lucide/svelte";
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as Select from "$lib/components/ui/select";
	import { onMount } from "svelte";

	let projects: Project[] = $state([]);
	let loading = $state(true);
	let newProjectOpen = $state(false);

	let repo = $state("https://github.com/heroku/node-js-sample");
	let name = $state("");
	let port = $state("");
	let gitToken = $state("");
	let buildCommand = $state("");
	let startCommand = $state("");
	let installCommand = $state("");
	let runtime = $state("nodejs");
	let envVars = $state([{ key: "", value: "" }]);
	let deploying = $state(false);
	let createdProject = $state<any>(null);

	onMount(async () => {
		if ($user) {
			const res = await listProjects();
			if (res) projects = res;
		}
		loading = false;
	});

	function addEnvVar() {
		envVars = [...envVars, { key: "", value: "" }];
	}

	function removeEnvVar(index: number) {
		envVars = envVars.filter((_, i) => i !== index);
	}

	async function handleEnvUpload(e: Event) {
		const input = e.target as HTMLInputElement;
		if (!input.files || input.files.length === 0) return;

		const file = input.files[0];
		const text = await file.text();
		
		const newVars: { key: string; value: string }[] = [];
		const lines = text.split('\n');

		for (const line of lines) {
			const trimmed = line.trim();
			if (!trimmed || trimmed.startsWith('#')) continue;

			const eqIdx = trimmed.indexOf('=');
			if (eqIdx === -1) continue;

			const key = trimmed.slice(0, eqIdx).trim();
			let value = trimmed.slice(eqIdx + 1).trim();

			// Remove surrounding quotes if present
			if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
				value = value.slice(1, -1);
			}

			if (key) {
				newVars.push({ key, value });
			}
		}

		if (newVars.length > 0) {
			// If the first item is empty, remove it
			if (envVars.length === 1 && !envVars[0].key && !envVars[0].value) {
				envVars = newVars;
			} else {
				envVars = [...envVars, ...newVars];
			}
		}
		
		input.value = ''; // Reset input
	}

	async function handleDeploy() {
		if (!repo || !name) return;
		deploying = true;

		const p = port ? parseInt(port) : undefined;
		const envMap: Record<string, string> = {};
		for (const e of envVars) {
			if (e.key) envMap[e.key] = e.value;
		}

		const res = await createProject(
			repo,
			name,
			p,
			envMap,
			gitToken,
			buildCommand,
			startCommand,
			installCommand,
			runtime,
		);
		if (res) {
			projects = [...projects, res];
			createdProject = res;
		}
		deploying = false;
	}

	$effect(() => {
		if (!newProjectOpen) {
			setTimeout(() => {
				createdProject = null;
				repo = "";
				name = "";
				port = "";
				gitToken = "";
				buildCommand = "";
				startCommand = "";
				installCommand = "";
				runtime = "nodejs";
				envVars = [{ key: "", value: "" }];
			}, 200);
		}
	});
</script>

<div class="container mx-auto py-10 px-4">
	{#if !$user}
		<div
			class="flex flex-col items-center justify-center min-h-[60vh] text-center space-y-8 animate-in fade-in zoom-in duration-500"
		>
			<div
				class="bg-primary/10 p-4 rounded-full mb-4 ring-1 ring-primary/20 shadow-[0_0_30px_-10px_rgba(255,255,255,0.3)]"
			>
				<Terminal class="w-12 h-12 text-primary" />
			</div>

			<div class="space-y-4 max-w-2xl">
				<h1 class="text-4xl md:text-6xl font-extrabold tracking-tight">
					Deploy with Clickploy
				</h1>
				<p class="text-xl text-muted-foreground leading-relaxed">
					Self-hosted PaaS made simple. Push your code, we handle the rest. No
					complex configs, just pure deployment power.
				</p>
			</div>

			<div class="flex flex-col sm:flex-row gap-4 w-full sm:w-auto pt-4">
				<Button
					href="/login"
					size="lg"
					class="min-w-[160px] text-lg h-12 shadow-lg hover:shadow-primary/25 transition-all"
				>
					Get Started
				</Button>
				<Button
					href="https://github.com/SirBlobby/Clickploy"
					variant="outline"
					size="lg"
					class="min-w-[160px] text-lg h-12"
				>
					<Github class="mr-2 h-5 w-5" /> GitHub
				</Button>
			</div>

			<div
				class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 pt-12 w-full max-w-5xl text-left"
			>
				<Card class="bg-card/50 border-muted">
					<CardHeader class="pb-2">
						<Activity class="w-8 h-8 text-primary mb-2" />
						<CardTitle class="text-lg">Zero Config</CardTitle>
					</CardHeader>
					<CardContent class="text-sm text-muted-foreground">
						Deploy from any Git repo without writing Dockerfiles or YAML.
					</CardContent>
				</Card>
				<Card class="bg-card/50 border-muted">
					<CardHeader class="pb-2">
						<Terminal class="w-8 h-8 text-primary mb-2" />
						<CardTitle class="text-lg">Docker Native</CardTitle>
					</CardHeader>
					<CardContent class="text-sm text-muted-foreground">
						Every app runs in its own isolated container for security.
					</CardContent>
				</Card>
				<Card class="bg-card/50 border-muted">
					<CardHeader class="pb-2">
						<ExternalLink class="w-8 h-8 text-primary mb-2" />
						<CardTitle class="text-lg">Auto Ports</CardTitle>
					</CardHeader>
					<CardContent class="text-sm text-muted-foreground">
						We automatically assign and manage ports for your services.
					</CardContent>
				</Card>
				<Card class="bg-card/50 border-muted">
					<CardHeader class="pb-2">
						<Settings class="w-8 h-8 text-primary mb-2" />
						<CardTitle class="text-lg">Full Control</CardTitle>
					</CardHeader>
					<CardContent class="text-sm text-muted-foreground">
						Self-hosted means you own your data, logs, and infrastructure.
					</CardContent>
				</Card>
				<Card class="bg-card/50 border-muted">
					<CardHeader class="pb-2">
						<Terminal class="w-8 h-8 text-primary mb-2" />
						<CardTitle class="text-lg">CLI Integration</CardTitle>
					</CardHeader>
					<CardContent class="text-sm text-muted-foreground">
						Manage your deployments from the terminal. (Coming Soon)
					</CardContent>
				</Card>
				<Card class="bg-card/50 border-muted">
					<CardHeader class="pb-2">
						<Github class="w-8 h-8 text-primary mb-2" />
						<CardTitle class="text-lg">Git Webhooks</CardTitle>
					</CardHeader>
					<CardContent class="text-sm text-muted-foreground">
						Automatically deploy when you push changes to your repository.
					</CardContent>
				</Card>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-between mb-8">
			<div>
				<h2 class="text-2xl font-bold tracking-tight">Overview</h2>
				<p class="text-muted-foreground">Manage your deployed applications.</p>
			</div>
			<div class="flex gap-2">
				<Button variant="outline" href="/activity">
					<Activity class="mr-2 h-4 w-4" /> Activity
				</Button>
				<Dialog.Root bind:open={newProjectOpen}>
					<Dialog.Trigger>
						<Button>
							<Plus class="mr-2 h-4 w-4" /> New Project
						</Button>
					</Dialog.Trigger>
					<Dialog.Content class="sm:max-w-[500px] max-h-[85vh] overflow-y-auto">
						<Dialog.Header>
							<Dialog.Title>Deploy Project</Dialog.Title>
							<Dialog.Description>
								Enter repository details to start a new deployment.
							</Dialog.Description>
						</Dialog.Header>
						<div class="grid gap-4 py-4">
							{#if createdProject}
								<div class="space-y-4">
									<div
										class="rounded-md bg-green-50 p-4 text-green-900 border border-green-200"
									>
										<h4 class="font-bold flex items-center gap-2">
											<div class="h-2 w-2 rounded-full bg-green-500"></div>
											Project Created Successfully!
										</h4>
									</div>

									<div class="space-y-2">
										<Label>Webhook URL</Label>
										<div class="flex items-center gap-2">
											<Input
												readonly
												value={`http://localhost:8080/webhooks/trigger?project_id=${createdProject.id}`}
											/>
											<Button
												variant="outline"
												size="icon"
												onclick={() =>
													navigator.clipboard.writeText(
														`http://localhost:8080/webhooks/trigger?project_id=${createdProject.id}`,
													)}
											>
												Copy
											</Button>
										</div>
										<p class="text-xs text-muted-foreground">
											Add this to your Git provider's webhook settings.
										</p>
									</div>

									<div class="space-y-2">
										<Label>Webhook Secret</Label>
										<div class="flex items-center gap-2">
											<Input readonly value={createdProject.webhook_secret} />
											<Button
												variant="outline"
												size="icon"
												onclick={() =>
													navigator.clipboard.writeText(
														createdProject.webhook_secret,
													)}
											>
												Copy
											</Button>
										</div>
									</div>

									<Button
										class="w-full"
										onclick={() => (newProjectOpen = false)}>Done</Button
									>
								</div>
							{:else}
								<div class="grid gap-2">
									<Label for="repo"
										>Repository URL <span class="text-red-500">*</span></Label
									>
									<Input
										id="repo"
										bind:value={repo}
										placeholder="https://github.com/username/repo"
									/>
								</div>
								<div class="grid gap-2">
									<Label for="name"
										>App Name <span class="text-red-500">*</span></Label
									>
									<Input id="name" placeholder="my-app" bind:value={name} />
								</div>
								<div class="grid gap-2">
									<Label for="port">Port (Optional)</Label>
									<Input id="port" placeholder="Auto" bind:value={port} />
								</div>

								<div class="grid gap-2">
									<Label>Runtime / Package Manager</Label>
									<Select.Root type="single" bind:value={runtime}>
										<Select.Trigger class="w-full">
											{runtime === "nodejs"
												? "Node.js (npm)"
												: runtime === "bun"
													? "Bun"
													: runtime === "deno"
														? "Deno"
														: runtime === "pnpm"
															? "Node.js (pnpm)"
															: "Select runtime"}
										</Select.Trigger>
										<Select.Content>
											<Select.Item value="nodejs">Node.js (npm)</Select.Item>
											<Select.Item value="bun">Bun</Select.Item>
											<Select.Item value="deno">Deno</Select.Item>
											<Select.Item value="pnpm">Node.js (pnpm)</Select.Item>
										</Select.Content>
									</Select.Root>
								</div>

								<div class="grid gap-2">
									<Label for="token">Git Token (Private Repo)</Label>
									<Input
										id="token"
										type="password"
										placeholder="ghp_..."
										bind:value={gitToken}
									/>
								</div>

								<div class="border-t pt-4 mt-2">
									<Collapsible.Root>
										<Collapsible.Trigger
											class="flex items-center justify-between w-full"
										>
											<div class="text-left">
												<Label class="text-base font-semibold cursor-pointer"
													>Build & Development Settings</Label
												>
												<p class="text-xs text-muted-foreground">
													Override default build commands.
												</p>
											</div>
											<ChevronsUpDown class="h-4 w-4 text-muted-foreground" />
										</Collapsible.Trigger>
										<Collapsible.Content class="space-y-3 pt-4">
											<div class="grid gap-2">
												<Label for="buildConfig">Build Command</Label>
												<Input
													id="buildConfig"
													placeholder="npm run build"
													bind:value={buildCommand}
												/>
											</div>
											<div class="grid gap-2">
												<Label for="startConfig">Start Command</Label>
												<Input
													id="startConfig"
													placeholder="npm run start"
													bind:value={startCommand}
												/>
											</div>
											<div class="grid gap-2">
												<Label for="installConfig">Install Command</Label>
												<Input
													id="installConfig"
													placeholder="npm install"
													bind:value={installCommand}
												/>
											</div>
										</Collapsible.Content>
									</Collapsible.Root>
								</div>

								<div class="border-t pt-4 mt-2">
									<Collapsible.Root>
										<Collapsible.Trigger
											class="flex items-center justify-between w-full"
										>
											<div class="text-left">
												<Label class="text-base font-semibold cursor-pointer"
													>Environment Variables</Label
												>
												<p class="text-xs text-muted-foreground">
													Configure runtime environment variables.
												</p>
											</div>
											<ChevronsUpDown class="h-4 w-4 text-muted-foreground" />
										</Collapsible.Trigger>
										<Collapsible.Content class="space-y-2 pt-4">
											{#each envVars as env, i}
												<div class="flex gap-2">
													<Input placeholder="Key" bind:value={env.key} />
													<Input placeholder="Value" bind:value={env.value} />
													<Button
														variant="ghost"
														size="icon"
														onclick={() => removeEnvVar(i)}
													>
														<LogOut class="h-4 w-4 rotate-45" />
													</Button>
												</div>
											{/each}
											<Button
												variant="outline"
												size="sm"
												onclick={addEnvVar}
												class="w-full"
											>
												<Plus class="mr-2 h-4 w-4" /> Add Variable
											</Button>
											
											<div class="relative">
												<input
													type="file"
													accept=".env,text/plain"
													class="hidden"
													id="env-upload"
													onchange={handleEnvUpload}
												/>
												<Button
													variant="secondary"
													size="sm"
													class="w-full"
													onclick={() => document.getElementById('env-upload')?.click()}
												>
													<Upload class="mr-2 h-4 w-4" /> Upload .env File
												</Button>
											</div>
										</Collapsible.Content>
									</Collapsible.Root>
								</div>
							{/if}
						</div>
						{#if !createdProject}
							<Dialog.Footer>
								<Button onclick={handleDeploy} disabled={deploying}>
									{#if deploying}
										<Loader2 class="mr-2 h-4 w-4 animate-spin" />
										Deploying...
									{:else}
										Deploy
									{/if}
								</Button>
							</Dialog.Footer>
						{/if}
					</Dialog.Content>
				</Dialog.Root>
			</div>
		</div>

		{#if loading}
			<div class="flex justify-center p-10">
				<Loader2 class="h-8 w-8 animate-spin" />
			</div>
		{:else if projects.length === 0}
			<div
				class="flex h-[450px] shrink-0 items-center justify-center rounded-md border border-dashed"
			>
				<div
					class="mx-auto flex max-w-[420px] flex-col items-center justify-center text-center"
				>
					<h3 class="mt-4 text-lg font-semibold">No projects created</h3>
					<p class="mb-4 mt-2 text-sm text-muted-foreground">
						You haven't deployed any projects yet.
					</p>
					<Button onclick={() => (newProjectOpen = true)}>
						<Plus class="mr-2 h-4 w-4" /> Add Project
					</Button>
				</div>
			</div>
		{:else}
			<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
				{#each projects as project}
					{@const latestDeployment = project.deployments?.[0]}
					{@const status = latestDeployment?.status || "unknown"}
					<Card
						class="group hover:shadow-lg transition-all duration-300 border-muted/60 hover:border-primary/50 cursor-pointer overflow-hidden relative"
					>
						<a href={`/projects/${project.id}`} class="block h-full">
							<CardHeader class="pb-3">
								<div class="flex items-start justify-between">
									<div class="space-y-1">
										<CardTitle class="text-xl flex items-center gap-2">
											{project.name}
										</CardTitle>
										<CardDescription class="flex items-center gap-1" >
											<Github class="h-3 w-3" />
											<a href={project.repo_url} target="_blank">{new URL(project.repo_url).pathname.slice(1)}</a>
										</CardDescription>
									</div>
									<span
										class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold capitalize
										{status === 'live'
											? 'border-transparent bg-green-500/15 text-green-500'
											: status === 'failed'
												? 'border-transparent bg-red-500/15 text-red-500'
												: status === 'building'
													? 'border-transparent bg-yellow-500/15 text-yellow-500'
													: 'border-transparent bg-gray-500/15 text-gray-400'}"
									>
										{status}
									</span>
								</div>
							</CardHeader>
							<CardContent>
								<div
									class="grid grid-cols-2 gap-4 text-sm text-muted-foreground mb-6"
								>
									<div class="flex flex-col gap-1">
										<span class="text-xs uppercase tracking-wider opacity-70"
											>Port</span
										>
										<span class="font-mono text-foreground">{project.port}</span
										>
									</div>
									<div class="flex flex-col gap-1">
										<span class="text-xs uppercase tracking-wider opacity-70"
											>Runtime</span
										>
										<div class="flex items-center gap-1.5">
											<span class="font-medium text-foreground capitalize"
												>{project.runtime || "nodejs"}</span
											>
										</div>
									</div>
									<div class="flex flex-col gap-1">
										<span class="text-xs uppercase tracking-wider opacity-70"
											>Deployments</span
										>
										<span class="font-medium text-foreground"
											>{project.deployments?.length || 0}</span
										>
									</div>
									<div class="flex flex-col gap-1">
										<span class="text-xs uppercase tracking-wider opacity-70"
											>Last Updated</span
										>
										<span class="font-medium text-foreground truncate">
											{latestDeployment
												? new Date(
														latestDeployment.CreatedAt,
													).toLocaleDateString()
												: "Never"}
										</span>
									</div>
								</div>

								<div class="flex gap-2 mt-auto">
									<Button
										variant="secondary"
										class="w-full group-hover:bg-primary group-hover:text-primary-foreground transition-colors"
										href={`/projects/${project.id}`}
									>
										Manage
									</Button>
									{#if status === "live" && latestDeployment?.url}
										<Button
											variant="outline"
											size="icon"
											href={latestDeployment.url}
											target="_blank"
											onclick={(e) => e.stopPropagation()}
											title="Visit App"
										>
											<ExternalLink class="h-4 w-4" />
										</Button>
									{/if}
								</div>
							</CardContent>
						</a>
					</Card>
				{/each}
			</div>
		{/if}
	{/if}
</div>
