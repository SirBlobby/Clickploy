<script lang="ts">
	import {
		Database,
		HardDrive,
		Plus,
		Server,
		AlertCircle,
	} from "@lucide/svelte";
	import { Button } from "$lib/components/ui/button";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
	} from "$lib/components/ui/card";
	import { Progress } from "$lib/components/ui/progress";

	import { getStorageStats, listDatabases, createDatabase } from "$lib/api";
	import { onMount } from "svelte";
	import { toast } from "svelte-sonner";

	let totalStorage = $state(0);
	let usedStorage = $state(0);
	let usagePercent = $derived(
		totalStorage > 0 ? (usedStorage / totalStorage) * 100 : 0,
	);

	let userDatabases = $state<any[]>([]);
	let loading = $state(true);

	const availableTypes = [
		{
			name: "SQLite",
			description: "Embedded, serverless database engine.",
			type: "sqlite",
			status: "Available",
		},
		{
			name: "PostgreSQL",
			description: "Advanced open source relational database.",
			type: "postgres",
			status: "Coming Soon",
		},
		{
			name: "Redis",
			description: "In-memory data structure store.",
			type: "redis",
			status: "Coming Soon",
		},
	];

	async function loadData() {
		const [stats, dbs] = await Promise.all([
			getStorageStats(),
			listDatabases(),
		]);
		if (stats) {
			totalStorage = stats.total_mb;
			usedStorage = stats.used_mb;
		}
		if (dbs) {
			userDatabases = dbs;
		}
		loading = false;
	}

	async function handleCreate(type: string) {
		const name = prompt("Enter database name:");
		if (!name) return;

		const res = await createDatabase(name, type);
		if (res) {
			toast.success("Database created successfully!");
			loadData();
		}
	}

	onMount(loadData);
</script>

<div class="container mx-auto py-10 px-4">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Storage</h1>
			<p class="text-muted-foreground">
				Manage databases and view storage usage.
			</p>
		</div>
		<Button>
			<Plus class="mr-2 h-4 w-4" /> Create Database
		</Button>
	</div>

	<div class="grid gap-6 md:grid-cols-3 mb-8">
		<Card class="md:col-span-3">
			<CardHeader>
				<CardTitle class="flex items-center gap-2">
					<HardDrive class="h-5 w-5" /> Storage Usage
				</CardTitle>
				<CardDescription>
					Total disk space used on the host machine.
				</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-4">
					<div class="flex items-center justify-between text-sm">
						<span class="font-medium"
							>{(usedStorage / 1024).toFixed(2)} GB used</span
						>
						<span class="text-muted-foreground"
							>{(totalStorage / 1024).toFixed(2)} GB total</span
						>
					</div>
					<Progress value={usagePercent} class="h-2" />
					<p class="text-xs text-muted-foreground">
						You are using {usagePercent.toFixed(1)}% of available storage.
					</p>
				</div>
			</CardContent>
		</Card>
	</div>

	<h2 class="text-xl font-semibold tracking-tight mb-4">Your Databases</h2>
	{#if userDatabases.length === 0}
		<div
			class="rounded-lg border border-dashed p-8 text-center text-muted-foreground mb-8"
		>
			No databases created yet.
		</div>
	{:else}
		<div class="grid gap-4 mb-8">
			{#each userDatabases as db}
				<div
					class="flex items-center justify-between rounded-lg border p-4 bg-muted/20"
				>
					<div class="flex items-center gap-4">
						<div class="rounded-full bg-blue-500/10 p-2">
							<Database class="h-6 w-6 text-blue-500" />
						</div>
						<div>
							<h3 class="font-semibold">{db.name}</h3>
							<p class="text-sm text-muted-foreground uppercase">
								{db.type} • {new Date(db.CreatedAt).toLocaleDateString()}
							</p>
						</div>
					</div>
					<span
						class="text-xs font-medium px-2.5 py-0.5 rounded-full bg-green-500/15 text-green-500"
					>
						{db.status}
					</span>
				</div>
			{/each}
		</div>
	{/if}

	<h2 class="text-xl font-semibold tracking-tight mb-4">Create New</h2>
	<div class="grid gap-4">
		{#each availableTypes as db}
			<div
				class="flex items-center justify-between rounded-lg border p-4 hover:bg-muted/50 transition-colors"
			>
				<div class="flex items-center gap-4">
					<div class="rounded-full bg-primary/10 p-2">
						<Server class="h-6 w-6 text-primary" />
					</div>
					<div>
						<h3 class="font-semibold">{db.name}</h3>
						<p class="text-sm text-muted-foreground">{db.description}</p>
					</div>
				</div>
				<div class="flex items-center gap-4">
					<span
						class="text-xs font-medium px-2.5 py-0.5 rounded-full {db.status ===
						'Available'
							? 'bg-green-500/15 text-green-500'
							: 'bg-yellow-500/15 text-yellow-500'}"
					>
						{db.status}
					</span>
					<Button
						variant="outline"
						disabled={db.status !== "Available"}
						onclick={() => handleCreate(db.type)}
					>
						Create
					</Button>
				</div>
			</div>
		{/each}
	</div>
</div>
