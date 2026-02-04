<script lang="ts">
	import {
		Database,
		HardDrive,
		Plus,
		Server,
		AlertCircle,
		Trash,
		Copy,
		Activity,
		Calendar,
		Cloud,
		Box,
		Check,
		Power,
		Play,
		RotateCw,
	} from "@lucide/svelte";
	import * as Tabs from "$lib/components/ui/tabs";
	import * as Dialog from "$lib/components/ui/dialog";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
		CardFooter,
	} from "$lib/components/ui/card";
	import { Progress } from "$lib/components/ui/progress";
	import { Badge } from "$lib/components/ui/badge";

	import {
		getStorageStats,
		listDatabases,
		createDatabase,
		deleteDatabase,
		getDatabaseCredentials,
		updateDatabaseCredentials,
		updateDatabase,
		stopDatabase,
		restartDatabase,
		type Database as DatabaseType,
	} from "$lib/api";
	import { onMount } from "svelte";
	import { toast } from "svelte-sonner";

	let totalStorage = $state(0);
	let usedStorage = $state(0);
	let usagePercent = $derived(
		totalStorage > 0 ? (usedStorage / totalStorage) * 100 : 0,
	);

	let userDatabases = $state<DatabaseType[]>([]);
	let loading = $state(true);

	// Credentials Dialog State (Creation)
	let showCredsDialog = $state(false);
	let newCreds = $state({
		uri: "",
		username: "",
		password: "",
		port: 0,
		name: "",
		host: "localhost",
	});

	// Management Dialog State
	let showManageDialog = $state(false);
	let manageDb = $state<DatabaseType | null>(null);
	let manageCreds = $state({
		username: "",
		password: "",
		uri: "",
		public_uri: "",
		loading: true,
		port: 0,
	});
	let isUpdatingCreds = $state(false);
	let isPowerAction = $state(false);

	// Creation Dialog State
	let showCreateDialog = $state(false);
	let selectedDbType = $state("");
	let newDbName = $state("");
	let isCreating = $state(false);

	const availableTypes = [
		{
			name: "SQLite",
			description: "Embedded, serverless database engine.",
			type: "sqlite",
			status: "Available",
			icon: Box,
			color: "text-blue-500",
			bgColor: "bg-blue-500/10",
		},
		{
			name: "MongoDB",
			description: "NoSQL document database.",
			type: "mongodb",
			status: "Available",
			icon: Database,
			color: "text-green-500",
			bgColor: "bg-green-500/10",
		},
		{
			name: "PostgreSQL",
			description: "Advanced open source relational database.",
			type: "postgres",
			status: "Coming Soon",
			icon: Cloud,
			color: "text-indigo-500",
			bgColor: "bg-indigo-500/10",
		},
		{
			name: "Redis",
			description: "In-memory data structure store.",
			type: "redis",
			status: "Coming Soon",
			icon: Activity,
			color: "text-red-500",
			bgColor: "bg-red-500/10",
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

	function initiateCreate(type: string) {
		selectedDbType = type;
		newDbName = "";
		showCreateDialog = true;
	}

	async function performCreate() {
		if (!newDbName.trim()) return;
		isCreating = true;

		try {
			const res: any = await createDatabase(newDbName, selectedDbType);
			showCreateDialog = false; // Close name input dialog

			if (res) {
				toast.success("Database created successfully!");
				if (res.uri) {
					newCreds = {
						uri: res.uri,
						username: res.username,
						password: res.password,
						port: res.database.port,
						name: res.database.name,
						host: window.location.hostname,
					};
					// Handle localhost vs public IP (simplified logic)
					if (
						newCreds.host !== "localhost" &&
						newCreds.host !== "127.0.0.1"
					) {
						newCreds.uri = newCreds.uri.replace(
							"@localhost",
							`@${newCreds.host}`,
						);
					}
					showCredsDialog = true;
				}
				loadData();
			}
		} catch (e) {
			toast.error("Failed to create database");
			console.error(e);
		} finally {
			isCreating = false;
		}
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		toast.success("Copied to clipboard");
	}

	async function handleDelete(id: number) {
		if (
			!confirm(
				"Are you sure you want to delete this database? This action cannot be undone.",
			)
		)
			return;

		const success = await deleteDatabase(id.toString());
		if (success) {
			toast.success("Database deleted successfully!");
			loadData();
		}
	}

	async function handleCardClick(db: DatabaseType) {
		if (db.type !== "mongodb") return; // Only Mongo supported for now for rich management
		manageDb = db;
		showManageDialog = true;
		manageCreds = { ...manageCreds, loading: true };

		const creds = await getDatabaseCredentials(db.ID.toString());
		if (creds) {
			manageCreds = {
				username: creds.username,
				password: creds.password,
				uri: creds.uri,
				public_uri: creds.public_uri,
				loading: false,
				port: db.port,
			};
		} else {
			manageCreds.loading = false;
			toast.error("Failed to load credentials");
		}
	}

	async function performUpdateCreds() {
		if (!manageDb) return;
		isUpdatingCreds = true;

		try {
			// Update Port if changed
			if (manageCreds.port !== manageDb.port) {
				const res = await updateDatabase(
					manageDb.ID.toString(),
					Number(manageCreds.port),
				);
				if (res) {
					toast.success("Port updated successfully!");
					await loadData();
					manageDb =
						userDatabases.find((d) => d.ID === manageDb!.ID) ||
						manageDb;
				}
			}

			// Update Credentials
			const success = await updateDatabaseCredentials(
				manageDb.ID.toString(),
				manageCreds.username,
				manageCreds.password,
			);

			if (success) {
				toast.success("Credentials updated successfully!");
				const creds = await getDatabaseCredentials(
					manageDb.ID.toString(),
				);
				if (creds) {
					manageCreds.uri = creds.uri;
				}
			}
		} catch (e) {
			console.error(e);
		} finally {
			isUpdatingCreds = false;
		}
	}

	async function performStopDatabase() {
		if (!manageDb) return;
		isPowerAction = true;
		const res = await stopDatabase(manageDb.ID.toString());
		if (res) {
			toast.success("Database stopped successfully");
			manageDb.status = "stopped";
			// Update the list
			userDatabases = userDatabases.map((d) =>
				d.ID === manageDb?.ID ? { ...d, status: "stopped" } : d,
			);
		}
		isPowerAction = false;
	}

	async function performRestartDatabase() {
		if (!manageDb) return;
		isPowerAction = true;
		const res = await restartDatabase(manageDb.ID.toString());
		if (res) {
			toast.success("Database restarted successfully");
			manageDb.status = "running";
			// Update the list
			userDatabases = userDatabases.map((d) =>
				d.ID === manageDb?.ID ? { ...d, status: "running" } : d,
			);
		}
		isPowerAction = false;
	}

	onMount(loadData);
</script>

<div class="container mx-auto py-8 px-4 max-w-7xl">
	<div
		class="flex flex-col md:flex-row md:items-center justify-between mb-8 gap-4"
	>
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Databases</h1>
			<p class="text-muted-foreground mt-1">
				Manage your database instances and storage volume.
			</p>
		</div>
	</div>

	<div class="grid gap-6 md:grid-cols-3 mb-10">
		<Card
			class="md:col-span-3 overflow-hidden border-l-4 border-l-primary/50"
		>
			<CardHeader class="pb-2">
				<CardTitle class="flex items-center gap-2 text-lg">
					<HardDrive class="h-5 w-5 text-primary" /> Storage Volume
				</CardTitle>
				<CardDescription>
					Total disk space usage across all your deployments and
					databases.
				</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-4">
					<div class="flex items-end justify-between">
						<div>
							<span class="text-2xl font-bold"
								>{(usedStorage / 1024).toFixed(2)} GB</span
							>
							<span class="text-sm text-muted-foreground ml-1"
								>used of {(totalStorage / 1024).toFixed(2)} GB</span
							>
						</div>
						<span class="font-medium text-sm text-muted-foreground"
							>{usagePercent.toFixed(1)}%</span
						>
					</div>
					<Progress value={usagePercent} class="h-2.5 w-full" />
				</div>
			</CardContent>
		</Card>
	</div>

	<div class="space-y-6">
		<div>
			<h2
				class="text-xl font-semibold tracking-tight mb-4 flex items-center gap-2"
			>
				<Server class="w-5 h-5 text-muted-foreground" />
				Active Databases
			</h2>

			{#if loading}
				<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
					{#each Array(3) as _}
						<div
							class="h-32 rounded-lg bg-muted/20 animate-pulse"
						></div>
					{/each}
				</div>
			{:else if userDatabases.length === 0}
				<div
					class="rounded-xl border border-dashed p-10 text-center bg-muted/10"
				>
					<div
						class="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-muted"
					>
						<Database class="h-6 w-6 text-muted-foreground" />
					</div>
					<h3 class="mt-4 text-lg font-semibold">No databases yet</h3>
					<p class="mb-4 text-sm text-muted-foreground">
						Create your first database to get started.
					</p>
				</div>
			{:else}
				<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
					{#each userDatabases as db}
						<button
							class="flex items-center justify-between p-4 rounded-xl border bg-card hover:shadow-md transition-all group text-left relative overflow-hidden"
							onclick={() => handleCardClick(db)}
							disabled={db.type !== "mongodb"}
						>
							<div class="flex items-center gap-4">
								<div
									class={`rounded-full p-2.5 ${
										db.type === "sqlite"
											? "bg-blue-500/10"
											: db.type === "mongodb"
												? "bg-green-500/10"
												: "bg-muted"
									}`}
								>
									{#if db.type === "sqlite"}
										<Box class="h-5 w-5 text-blue-500" />
									{:else if db.type === "mongodb"}
										<Database
											class="h-5 w-5 text-green-500"
										/>
									{:else}
										<Database
											class="h-5 w-5 text-muted-foreground"
										/>
									{/if}
								</div>
								<div>
									<h3 class="font-semibold text-base">
										{db.name}
									</h3>
									<div
										class="flex items-center gap-2 text-sm text-muted-foreground"
									>
										<span class="capitalize">{db.type}</span
										>
										<span>•</span>
										<div class="flex items-center gap-1.5">
											<div
												class={`h-2 w-2 rounded-full ${db.status === "running" || db.status === "Available" ? "bg-green-500" : "bg-yellow-500"}`}
											></div>
											<span class="capitalize"
												>{db.status}</span
											>
										</div>
									</div>
								</div>
							</div>

							<div class="flex items-center gap-4">
								{#if db.port > 0}
									<Badge
										variant="secondary"
										class="font-mono"
									>
										:{db.port}
									</Badge>
								{/if}
								<Button
									variant="ghost"
									size="icon"
									class="text-muted-foreground hover:text-red-500 z-10"
									onclick={(e) => {
										e.stopPropagation();
										handleDelete(db.ID);
									}}
								>
									<Trash class="h-4 w-4" />
								</Button>
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>

		<div class="pt-8">
			<h2
				class="text-xl font-semibold tracking-tight mb-4 flex items-center gap-2"
			>
				<Plus class="w-5 h-5 text-muted-foreground" />
				Create New Database
			</h2>
			<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
				{#each availableTypes as db}
					<button
						class="flex flex-col items-start text-left rounded-xl border p-4 hover:bg-muted/30 hover:border-primary/50 transition-all disabled:opacity-50 disabled:pointer-events-none group"
						disabled={db.status !== "Available"}
						onclick={() => initiateCreate(db.type)}
					>
						<div
							class="flex w-full items-center justify-between mb-3"
						>
							<div class={`rounded-full p-2.5 ${db.bgColor}`}>
								<db.icon class={`h-5 w-5 ${db.color}`} />
							</div>
							{#if db.status === "Available"}
								<Badge
									variant="secondary"
									class="bg-primary/10 text-primary hover:bg-primary/20"
									>Available</Badge
								>
							{:else}
								<Badge
									variant="outline"
									class="text-muted-foreground">Soon</Badge
								>
							{/if}
						</div>
						<h3 class="font-semibold text-lg">{db.name}</h3>
						<p
							class="text-sm text-muted-foreground mt-1 leading-snug"
						>
							{db.description}
						</p>
					</button>
				{/each}
			</div>
		</div>
	</div>
</div>

<!-- Creation Dialog -->
<Dialog.Root bind:open={showCreateDialog}>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title
				>Create {availableTypes.find((t) => t.type === selectedDbType)
					?.name} Database</Dialog.Title
			>
			<Dialog.Description>
				Enter a unique name for your new database instance.
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="name" class="text-right">Name</Label>
				<Input
					id="name"
					bind:value={newDbName}
					placeholder="my-database-1"
					class="col-span-3"
					autofocus
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => (showCreateDialog = false)}
				>Cancel</Button
			>
			<Button
				onclick={performCreate}
				disabled={isCreating || !newDbName.trim()}
			>
				{isCreating ? "Creating..." : "Create Database"}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<!-- Credentials Dialog -->
<Dialog.Root bind:open={showCredsDialog}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title class="flex items-center gap-2">
				<Check class="h-5 w-5 text-green-500" /> Database Created
			</Dialog.Title>
			<Dialog.Description>
				Your database is ready. These variables are only shown once, so
				please copy them now.
			</Dialog.Description>
		</Dialog.Header>
		<div class="flex flex-col space-y-4 py-4">
			<div class="space-y-2">
				<Label>Connection URI</Label>
				<div class="flex items-center gap-2">
					<Input
						readonly
						value={newCreds.uri}
						class="font-mono bg-muted text-xs"
					/>
					<Button
						variant="outline"
						size="icon"
						onclick={() => copyToClipboard(newCreds.uri)}
					>
						<Copy class="h-4 w-4" />
					</Button>
				</div>
			</div>
			<div class="grid grid-cols-2 gap-4">
				<div class="space-y-2">
					<Label>Username</Label>
					<div class="flex items-center gap-2">
						<Input
							readonly
							value={newCreds.username}
							class="font-mono bg-muted text-xs"
						/>
						<Button
							variant="outline"
							size="icon"
							onclick={() => copyToClipboard(newCreds.username)}
						>
							<Copy class="h-4 w-4" />
						</Button>
					</div>
				</div>
				<div class="space-y-2">
					<Label>Password</Label>
					<div class="flex items-center gap-2">
						<Input
							readonly
							value={newCreds.password}
							type="password"
							class="font-mono bg-muted text-xs"
						/>
						<Button
							variant="outline"
							size="icon"
							onclick={() => copyToClipboard(newCreds.password)}
						>
							<Copy class="h-4 w-4" />
						</Button>
					</div>
				</div>
			</div>

			<div
				class="rounded-md bg-yellow-500/10 p-4 border border-yellow-500/20"
			>
				<div class="flex items-start gap-4">
					<AlertCircle class="h-5 w-5 text-yellow-500 mt-0.5" />
					<div class="space-y-1">
						<p class="text-sm font-medium text-yellow-500">
							Important
						</p>
						<p
							class="text-sm text-yellow-600/90 dark:text-yellow-500/90"
						>
							This database is accessible via port <span
								class="font-mono font-bold"
								>{newCreds.port}</span
							>. Make sure to allow this port in your firewall if
							accessing externally.
						</p>
					</div>
				</div>
			</div>
		</div>
		<Dialog.Footer>
			<Button onclick={() => (showCredsDialog = false)}>Done</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<!-- Management Dialog -->
<Dialog.Root bind:open={showManageDialog}>
	<Dialog.Content class="sm:max-w-[600px]">
		<Dialog.Header>
			<Dialog.Title class="flex items-center gap-2 text-xl">
				<Database class="h-5 w-5 text-primary" /> Manage {manageDb?.name}
			</Dialog.Title>
			<Dialog.Description>
				Configure connection details, credentials, and instance power
				state.
			</Dialog.Description>
		</Dialog.Header>

		{#if manageCreds.loading}
			<div class="flex items-center justify-center py-12">
				<div
					class="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full"
				></div>
			</div>
		{:else}
			<Tabs.Root value="connection" class="w-full">
				<Tabs.List class="grid w-full grid-cols-2">
					<Tabs.Trigger value="connection">Connection</Tabs.Trigger>
					<Tabs.Trigger value="settings"
						>Settings & Danger</Tabs.Trigger
					>
				</Tabs.List>

				<Tabs.Content value="connection" class="space-y-4 py-4">
					<div
						class="rounded-lg border bg-card text-card-foreground shadow-sm p-6 space-y-6"
					>
						<div class="flex items-center justify-between">
							<div class="space-y-1">
								<h3 class="font-medium leading-none">Status</h3>
								<p class="text-sm text-muted-foreground">
									Current state of the database container.
								</p>
							</div>
							<Badge
								variant={manageDb?.status === "running"
									? "default"
									: "secondary"}
								class={`${manageDb?.status === "running" ? "bg-green-500 hover:bg-green-600" : ""}`}
							>
								{manageDb?.status}
							</Badge>
						</div>

						<div class="space-y-2">
							<Label>Connection string</Label>
							<div class="relative">
								<Input
									readonly
									value={manageCreds.uri}
									class="pr-10 font-mono text-sm"
								/>
								<Button
									variant="ghost"
									size="icon"
									class="absolute right-0 top-0 h-full px-3 text-muted-foreground hover:text-foreground"
									onclick={() =>
										copyToClipboard(manageCreds.uri)}
								>
									<Copy class="h-4 w-4" />
								</Button>
							</div>
						</div>

						<div class="grid grid-cols-2 gap-4">
							<div class="space-y-2">
								<Label>Host</Label>
								<div
									class="p-2 rounded-md border bg-muted font-mono text-sm"
								>
									localhost
								</div>
							</div>
							<div class="space-y-2">
								<Label>Port</Label>
								<Input
									type="number"
									class="font-mono text-sm"
									bind:value={manageCreds.port}
								/>
							</div>
						</div>
					</div>

					<div class="flex justify-end">
						<Button
							onclick={performUpdateCreds}
							disabled={isUpdatingCreds}
						>
							{isUpdatingCreds
								? "Saving Changes..."
								: "Save Changes"}
						</Button>
					</div>
				</Tabs.Content>

				<Tabs.Content value="settings" class="space-y-4 py-4">
					<div class="rounded-lg border p-4 space-y-4">
						<h3 class="font-medium text-sm">Credentials</h3>
						<div class="grid gap-4">
							<div class="grid gap-2">
								<Label>Username</Label>
								<Input
									bind:value={manageCreds.username}
									class="font-mono"
								/>
							</div>
							<div class="grid gap-2">
								<Label>Password</Label>
								<div class="relative">
									<Input
										bind:value={manageCreds.password}
										type="text"
										class="pr-10 font-mono"
									/>
									<Button
										variant="ghost"
										size="icon"
										class="absolute right-0 top-0 h-full px-3"
										onclick={() =>
											copyToClipboard(
												manageCreds.password,
											)}
									>
										<Copy class="h-4 w-4" />
									</Button>
								</div>
							</div>
							<Button
								onclick={performUpdateCreds}
								disabled={isUpdatingCreds}
								variant="default"
							>
								Update Credentials
							</Button>
						</div>
					</div>

					<div
						class="rounded-lg border border-red-200 dark:border-red-900 bg-red-50 dark:bg-red-950/10 p-4 space-y-4"
					>
						<div>
							<h3
								class="font-medium text-sm text-red-600 dark:text-red-400"
							>
								Power Actions
							</h3>
							<p
								class="text-sm text-red-600/70 dark:text-red-400/70"
							>
								Manage the running state of your database.
							</p>
						</div>

						<div class="flex gap-2">
							<Button
								variant="outline"
								class="border-red-200 hover:bg-red-100 hover:text-red-600 dark:border-red-900 dark:hover:bg-red-900/30"
								disabled={isPowerAction ||
									manageDb?.status === "stopped"}
								onclick={performStopDatabase}
							>
								<Power class="h-4 w-4 mr-2" />
								Stop
							</Button>
							<Button
								variant="outline"
								class="border-red-200 hover:bg-red-100 hover:text-red-600 dark:border-red-900 dark:hover:bg-red-900/30"
								disabled={isPowerAction}
								onclick={performRestartDatabase}
							>
								{#if manageDb?.status === "stopped"}
									<Play class="h-4 w-4 mr-2" /> Start
								{:else}
									<RotateCw class="h-4 w-4 mr-2" /> Restart
								{/if}
							</Button>
						</div>
					</div>
				</Tabs.Content>
			</Tabs.Root>
		{/if}
	</Dialog.Content>
</Dialog.Root>
