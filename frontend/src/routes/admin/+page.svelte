<script lang="ts">
	import { getAdminUsers, deleteAdminUser, getAdminStats } from "$lib/api";
	import { user, type User } from "$lib/auth";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
	} from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import {
		Table,
		TableBody,
		TableCell,
		TableHead,
		TableHeader,
		TableRow,
	} from "$lib/components/ui/table";
	import { Loader2, Trash2, Shield, Users, Box, Layers } from "@lucide/svelte";
	import { toast } from "svelte-sonner";

	let loading = true;
	let users: any[] = [];
	let stats: any = null;

	onMount(async () => {
		if (!$user || !$user.is_admin) {
			toast.error("Unauthorized");
			goto("/");
			return;
		}

		await loadData();
	});

	async function loadData() {
		loading = true;
		const [u, s] = await Promise.all([getAdminUsers(), getAdminStats()]);
		users = u || [];
		stats = s;
		loading = false;
	}

	async function handleDeleteUser(id: string) {
		if (!confirm("Are you sure you want to delete this user?")) return;
		const success = await deleteAdminUser(id);
		if (success) {
			toast.success("User deleted");
			users = users.filter((u) => u.id !== id);
		}
	}
</script>

<div class="container mx-auto py-10 px-4">
	<div class="flex items-center justify-between mb-8">
		<div>
			<h2 class="text-3xl font-bold tracking-tight">Admin Dashboard</h2>
			<p class="text-muted-foreground">System overview and user management.</p>
		</div>
	</div>

	{#if loading}
		<div class="flex justify-center p-20">
			<Loader2 class="h-8 w-8 animate-spin" />
		</div>
	{:else}
		<!-- Stats Cards -->
		{#if stats}
			<div class="grid gap-4 md:grid-cols-3 mb-8">
				<Card>
					<CardHeader class="flex flex-row items-center justify-between pb-2">
						<CardTitle class="text-sm font-medium">Total Users</CardTitle>
						<Users class="h-4 w-4 text-muted-foreground" />
					</CardHeader>
					<CardContent>
						<div class="text-2xl font-bold">{stats.users}</div>
					</CardContent>
				</Card>
				<Card>
					<CardHeader class="flex flex-row items-center justify-between pb-2">
						<CardTitle class="text-sm font-medium">Total Projects</CardTitle>
						<Box class="h-4 w-4 text-muted-foreground" />
					</CardHeader>
					<CardContent>
						<div class="text-2xl font-bold">{stats.projects}</div>
					</CardContent>
				</Card>
				<Card>
					<CardHeader class="flex flex-row items-center justify-between pb-2">
						<CardTitle class="text-sm font-medium">Total Deployments</CardTitle>
						<Layers class="h-4 w-4 text-muted-foreground" />
					</CardHeader>
					<CardContent>
						<div class="text-2xl font-bold">{stats.deployments}</div>
					</CardContent>
				</Card>
			</div>
		{/if}

		<!-- Users Table -->
		<Card>
			<CardHeader>
				<CardTitle>Users</CardTitle>
				<CardDescription>Manage registered users.</CardDescription>
			</CardHeader>
			<CardContent>
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead>ID</TableHead>
							<TableHead>Name</TableHead>
							<TableHead>Email</TableHead>
							<TableHead>Projects</TableHead>
							<TableHead>Role</TableHead>
							<TableHead class="text-right">Actions</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each users as u}
							<TableRow>
								<TableCell>{u.id}</TableCell>
								<TableCell class="font-medium">{u.name}</TableCell>
								<TableCell>{u.email}</TableCell>
								<TableCell>{u.projects?.length || 0}</TableCell>
								<TableCell>
									{#if u.is_admin}
										<span
											class="inline-flex items-center rounded-full border border-transparent bg-primary/10 px-2.5 py-0.5 text-xs font-semibold text-primary"
										>
											<Shield class="mr-1 h-3 w-3" /> Admin
										</span>
									{:else}
										<span class="text-muted-foreground">User</span>
									{/if}
								</TableCell>
								<TableCell class="text-right">
									{#if !u.is_admin}
										<Button
											variant="ghost"
											size="icon"
											class="text-destructive hover:text-destructive/90"
											onclick={() => handleDeleteUser(u.id)}
										>
											<Trash2 class="h-4 w-4" />
										</Button>
									{/if}
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</CardContent>
		</Card>
	{/if}
</div>
