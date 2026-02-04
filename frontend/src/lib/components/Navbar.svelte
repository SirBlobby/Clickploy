<script lang="ts">
	import { user, logout } from "$lib/auth";
	import { Button } from "$lib/components/ui/button";
	import {
		Terminal,
		Settings,
		LogOut,
		Activity,
		LayoutDashboard,
		Menu,
		X,
		Rocket,
		Network,
		Database,
	} from "@lucide/svelte";
	import { page } from "$app/stores";
	import * as Sheet from "$lib/components/ui/sheet";

	let mobileOpen = $state(false);

	function isActive(path: string) {
		return $page.url.pathname === path ? "bg-secondary" : "hover:bg-accent/50";
	}
</script>

<header
	class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur-sm"
>
	<div class="container mx-auto flex h-14 items-center justify-between px-4">
		<div class="flex items-center gap-6">
			<a href="/" class="flex items-center gap-2 font-bold text-xl">
				<Terminal class="h-6 w-6" />
				<span>Clickploy</span>
			</a>

			{#if $user}
				<nav class="hidden md:flex items-center gap-2">
					<Button variant="ghost" size="sm" href="/" class={isActive("/")}>
						<LayoutDashboard class="mr-2 h-4 w-4" /> Overview
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/deployments"
						class={isActive("/deployments")}
					>
						<Rocket class="mr-2 h-4 w-4" /> Deployments
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/network"
						class={isActive("/network")}
					>
						<Network class="mr-2 h-4 w-4" /> Network
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/activity"
						class={isActive("/activity")}
					>
						<Activity class="mr-2 h-4 w-4" /> Activity
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/storage"
						class={isActive("/storage")}
					>
						<Database class="mr-2 h-4 w-4" /> Storage
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/settings"
						class={isActive("/settings")}
					>
						<Settings class="mr-2 h-4 w-4" /> Settings
					</Button>
				</nav>
			{/if}
		</div>

		{#if $user}
			<div class="flex items-center gap-4">
				<div class="hidden md:flex items-center gap-2">
					<div
						class="h-8 w-8 rounded-full bg-linear-to-tr from-primary to-purple-500"
					></div>
					<span class="text-sm font-medium">{$user.name}</span>
				</div>
				<Button
					variant="ghost"
					size="icon"
					onclick={logout}
					title="Log out"
					class="hidden md:flex"
				>
					<LogOut class="h-4 w-4" />
				</Button>

				<div class="md:hidden">
					<Sheet.Root bind:open={mobileOpen}>
						<Sheet.Trigger>
							<Button variant="ghost" size="icon">
								<Menu class="h-5 w-5" />
							</Button>
						</Sheet.Trigger>
						<Sheet.Content side="right">
							<Sheet.Header>
								<Sheet.Title>Menu</Sheet.Title>
							</Sheet.Header>
							<div class="grid gap-4 py-4">
								<a
									href="/"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<LayoutDashboard class="h-5 w-5" /> Overview
								</a>
								<a
									href="/deployments"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Rocket class="h-5 w-5" /> Deployments
								</a>
								<a
									href="/network"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Network class="h-5 w-5" /> Network
								</a>
								<a
									href="/activity"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Activity class="h-5 w-5" /> Activity
								</a>
								<a
									href="/storage"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Database class="h-5 w-5" /> Storage
								</a>
								<a
									href="/settings"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Settings class="h-5 w-5" /> Settings
								</a>
								<div class="border-t my-2"></div>
								<div class="flex items-center gap-2 py-2">
									<div
										class="h-6 w-6 rounded-full bg-linear-to-tr from-primary to-purple-500"
									></div>
									<span class="font-medium">{$user.name}</span>
								</div>
								<Button
									variant="ghost"
									class="justify-start gap-2 px-0"
									onclick={logout}
								>
									<LogOut class="h-5 w-5" /> Log out
								</Button>
							</div>
						</Sheet.Content>
					</Sheet.Root>
				</div>
			</div>
		{:else}
			<div class="flex gap-2">
				<Button variant="ghost" size="sm" href="/login">Login</Button>
				<Button size="sm" href="/register">Get Started</Button>
			</div>
		{/if}
	</div>
</header>
