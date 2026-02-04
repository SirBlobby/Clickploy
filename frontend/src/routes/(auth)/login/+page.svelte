<script lang="ts">
	import { loginUser } from "$lib/api";
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

	let email = $state("");
	let password = $state("");
	let loading = $state(false);

	async function handleSubmit() {
		if (!email || !password) return;
		loading = true;
		const res = await loginUser(email, password);
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
			<CardTitle>Sign In</CardTitle>
			<CardDescription>Enter your email to access your account</CardDescription>
		</CardHeader>
		<CardContent>
			<div class="grid gap-4">
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
				Sign In
			</Button>
			<div class="text-center text-sm text-muted-foreground">
				Don't have an account? <a
					href="/register"
					class="underline hover:text-primary">Sign up</a
				>
			</div>
		</CardFooter>
	</Card>
</div>
