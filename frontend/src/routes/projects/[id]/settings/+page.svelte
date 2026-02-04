<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import { getProject, type Project, updateProjectEnv } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
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
	} from "@lucide/svelte";
	import { toast } from "svelte-sonner";

	let project = $state<Project | null>(null);
	let loading = $state(true);
	let showSecret = $state(false);

	let tempEnvVars = $state<{ key: string; value: string }[]>([]);
	let isDirty = $state(false);

	onMount(async () => {
		const id = $page.params.id;
		if (id) {
			const res = await getProject(id);
			if (res) {
				project = res;
				initEnvVars();
			}
		}
		loading = false;
	});

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
		const success = await updateProjectEnv(project.ID.toString(), envMap);
		loading = false;

		if (success) {
			toast.success("Environment variables updated successfully");
			const res = await getProject(project.ID.toString());
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
		const displayUrl = `http://localhost:8080/webhooks/trigger?project_id=${project.ID}`;
		navigator.clipboard.writeText(displayUrl);
		toast.success("Webhook URL copied");
	}
</script>

{#if loading && !project}
	<div class="flex justify-center p-10">
		<Loader2 class="h-8 w-8 animate-spin" />
	</div>
{:else if project}
	<div class="space-y-6 max-w-4xl">
		<Card class="border-border/60">
			<CardHeader class="pb-4">
				<div class="flex items-center justify-between">
					<div>
						<CardTitle class="text-xl font-bold"
							>Environment Variables</CardTitle
						>
						<CardDescription class="mt-1 text-sm text-muted-foreground">
							Configure runtime environment variables.
						</CardDescription>
					</div>
					<Button variant="ghost" size="sm" onclick={toggleSecret} class="h-8">
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

				<Button
					variant="outline"
					class="w-full h-11 border-dashed border-border/60 hover:bg-muted/50"
					onclick={addEnvVar}
				>
					<Plus class="h-4 w-4 mr-2" />
					Add Variable
				</Button>
			</CardContent>
			<CardFooter class="border-t px-4 flex justify-end">
				<Button onclick={saveEnvVars} disabled={!isDirty || loading} size="sm">
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
					Trigger deployments automatically when you push to your repository.
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label class="text-sm">Webhook URL</Label>
					<div class="flex items-center gap-2">
						<Input
							readonly
							value={`http://localhost:8080/webhooks/trigger?project_id=${project.ID}`}
							class="bg-muted font-mono text-xs"
						/>
						<Button variant="outline" size="icon" onclick={copyWebhook}>
							<Copy class="h-4 w-4" />
						</Button>
					</div>
				</div>
				<div class="space-y-2">
					<Label class="text-sm">Webhook Secret</Label>
					<Input
						type="password"
						readonly
						value={project.webhook_secret}
						class="bg-muted font-mono text-xs"
					/>
				</div>
			</CardContent>
		</Card>

		<Card class="border-destructive/50 bg-destructive/5">
			<CardHeader>
				<CardTitle class="text-destructive text-lg">Danger Zone</CardTitle>
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
