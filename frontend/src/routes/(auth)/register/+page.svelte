<script lang="ts">
	import { registerUser } from "$lib/api";
	import { login } from "$lib/auth";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import {
		Card,
		CardContent,
		CardDescription,
		CardFooter,
		CardHeader,
		CardTitle,
	} from "$lib/components/ui/card";
	import { goto } from "$app/navigation";
	import { Loader2 } from "@lucide/svelte";

	let name = $state("");
	let email = $state("");
	let password = $state("");
	let loading = $state(false);

	async function handleSubmit() {
		if (!email || !password || !name) return;
		loading = true;
		const res = await registerUser(email, password, name);
		if (res) {
			login(res.user, res.token);
			goto("/");
		}
		loading = false;
	}
</script>

<div class="flex h-screen items-center justify-center bg-muted/50">
	<Card class="w-[350px]">
		<CardHeader>
			<CardTitle>Create Account</CardTitle>
			<CardDescription>Enter your email to create an account</CardDescription>
		</CardHeader>
		<CardContent>
			<div class="grid gap-4">
				<div class="grid gap-2">
					<Label for="name">Name</Label>
					<Input id="name" placeholder="John Doe" bind:value={name} />
				</div>
				<div class="grid gap-2">
					<Label for="email">Email</Label>
					<Input
						id="email"
						type="email"
						placeholder="m@example.com"
						bind:value={email}
					/>
				</div>
				<div class="grid gap-2">
					<Label for="password">Password</Label>
					<Input id="password" type="password" bind:value={password} />
				</div>
			</div>
		</CardContent>
		<CardFooter class="flex flex-col gap-4">
			<Button class="w-full" onclick={handleSubmit} disabled={loading}>
				{#if loading}
					<Loader2 class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Sign Up
			</Button>
			<div class="text-center text-sm text-muted-foreground">
				Already have an account? <a
					href="/login"
					class="underline hover:text-primary">Sign in</a
				>
			</div>
		</CardFooter>
	</Card>
</div>
