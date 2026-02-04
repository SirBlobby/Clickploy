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
		Shield,
		Book,
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
					<Button
						variant="ghost"
						size="sm"
						href="/"
						class={`group ${isActive("/")}`}
					>
						<LayoutDashboard class="h-4 w-4" />
						<span
							class={`max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap opacity-0 group-hover:opacity-100 group-hover:ml-2 ${$page.url.pathname === "/" ? "max-w-xs opacity-100 ml-2" : ""}`}
						>
							Overview
						</span>
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/deployments"
						class={`group ${isActive("/deployments")}`}
					>
						<Rocket class="h-4 w-4" />
						<span
							class={`max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap opacity-0 group-hover:opacity-100 group-hover:ml-2 ${$page.url.pathname === "/deployments" ? "max-w-xs opacity-100 ml-2" : ""}`}
						>
							Deployments
						</span>
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/network"
						class={`group ${isActive("/network")}`}
					>
						<Network class="h-4 w-4" />
						<span
							class={`max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap opacity-0 group-hover:opacity-100 group-hover:ml-2 ${$page.url.pathname === "/network" ? "max-w-xs opacity-100 ml-2" : ""}`}
						>
							Network
						</span>
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/activity"
						class={`group ${isActive("/activity")}`}
					>
						<Activity class="h-4 w-4" />
						<span
							class={`max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap opacity-0 group-hover:opacity-100 group-hover:ml-2 ${$page.url.pathname === "/activity" ? "max-w-xs opacity-100 ml-2" : ""}`}
						>
							Activity
						</span>
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/storage"
						class={`group ${isActive("/storage")}`}
					>
						<Database class="h-4 w-4" />
						<span
							class={`max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap opacity-0 group-hover:opacity-100 group-hover:ml-2 ${$page.url.pathname === "/storage" ? "max-w-xs opacity-100 ml-2" : ""}`}
						>
							Storage
						</span>
					</Button>
					<Button
						variant="ghost"
						size="sm"
						href="/docs"
						class={`group ${isActive("/docs")}`}
					>
						<Book class="h-4 w-4" />
						<span
							class={`max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap opacity-0 group-hover:opacity-100 group-hover:ml-2 ${$page.url.pathname.startsWith("/docs") ? "max-w-xs opacity-100 ml-2" : ""}`}
						>
							Docs
						</span>
					</Button>
				</nav>
			{/if}
		</div>

		{#if $user}
			<div class="flex items-center gap-4">
				<nav class="hidden md:flex items-center gap-2 mr-2">
					<Button
						variant="ghost"
						size="sm"
						href="/settings"
						class={isActive("/settings")}
					>
						<Settings class="h-4 w-4" />
					</Button>
					{#if $user.is_admin}
						<Button
							variant="ghost"
							size="sm"
							href="/admin"
							class={isActive("/admin")}
						>
							<Shield class="h-4 w-4" />
						</Button>
					{/if}
				</nav>
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
									href="/docs"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Book class="h-5 w-5" /> Docs
								</a>
								<a
									href="/settings"
									class="flex items-center gap-2 py-2 text-lg font-medium"
									onclick={() => (mobileOpen = false)}
								>
									<Settings class="h-5 w-5" /> Settings
								</a>
								{#if $user.is_admin}
									<a
										href="/admin"
										class="flex items-center gap-2 py-2 text-lg font-medium"
										onclick={() => (mobileOpen = false)}
									>
										<Shield class="h-5 w-5" /> Admin
									</a>
								{/if}
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
				<Button variant="ghost" size="sm" href="/docs">Docs</Button>
				<Button variant="ghost" size="sm" href="/login">Login</Button>
				<Button size="sm" href="/register">Get Started</Button>
			</div>
		{/if}
	</div>
</header>
