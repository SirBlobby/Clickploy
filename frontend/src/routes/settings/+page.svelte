<script lang="ts">
	import { user } from "$lib/auth";
	import { updateProfile, updatePassword } from "$lib/api";
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
	import { ArrowLeft } from "@lucide/svelte";
	import { toast } from "svelte-sonner";

	let name = $state($user?.name || "");
	let email = $state($user?.email || "");
	let oldPassword = $state("");
	let newPassword = $state("");
	let loading = $state(false);

	async function handleUpdateProfile(e: Event) {
		e.preventDefault();
		loading = true;
		const res = await updateProfile(name, email);
		if (res) {
			toast.success("Profile updated");
			user.update((u) => {
				if (!u) return null;
				return { ...u, name: res.name, email: res.email };
			});
		}
		loading = false;
	}

	async function handleUpdatePassword(e: Event) {
		e.preventDefault();
		loading = true;
		const res = await updatePassword(oldPassword, newPassword);
		if (res) {
			toast.success("Password updated");
			oldPassword = "";
			newPassword = "";
		}
		loading = false;
	}
</script>

<div class="container mx-auto py-10 px-4 max-w-2xl">
	<div class="mb-6">
		<h1 class="text-3xl font-bold tracking-tight">Account Settings</h1>
		<p class="text-muted-foreground">Manage your profile and preferences.</p>
	</div>

	{#if $user}
		<div class="space-y-6">
			<Card>
				<CardHeader>
					<CardTitle>Profile</CardTitle>
					<CardDescription>Your personal information.</CardDescription>
				</CardHeader>
				<CardContent class="space-y-4">
					<form class="space-y-4" onsubmit={handleUpdateProfile}>
						<div class="space-y-2">
							<Label>Name</Label>
							<Input bind:value={name} />
						</div>
						<div class="space-y-2">
							<Label>Email</Label>
							<Input bind:value={email} />
						</div>
						<Button type="submit" disabled={loading}>Update Profile</Button>
					</form>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle>Security</CardTitle>
					<CardDescription>Change your password.</CardDescription>
				</CardHeader>
				<CardContent>
					<form class="space-y-4" onsubmit={handleUpdatePassword}>
						<div class="space-y-2">
							<Label>Current Password</Label>
							<Input type="password" bind:value={oldPassword} required />
						</div>
						<div class="space-y-2">
							<Label>New Password</Label>
							type="password" bind:value={newPassword}
							required minlength={6}
							/>
						</div>
						<Button type="submit" variant="secondary" disabled={loading}
							>Change Password</Button
						>
					</form>
				</CardContent>
			</Card>
		</div>
	{:else}
		<p>Please log in.</p>
	{/if}
</div>
