<script lang="ts">
	import {
		Github,
		Twitter,
		CheckCircle2,
		AlertCircle,
		Terminal,
	} from "@lucide/svelte";
	import { onMount } from "svelte";
	import { getSystemStatus } from "$lib/api";

	let status = $state("Checking...");
	let version = $state("");
	let isNormal = $state(true);

	onMount(async () => {
		const res = await getSystemStatus();
		if (res) {
			status = res.status;
			version = res.version;
			isNormal = true;
		} else {
			status = "System Unavailable";
			isNormal = false;
		}
	});
</script>

<footer
	class="border-t py-6 md:py-0 bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60"
>
	<div
		class="container flex flex-col items-center justify-center gap-6 md:h-16 md:flex-row px-4"
	>
		<div class="flex flex-col items-center gap-6 md:flex-row">
			<a href="/" class="flex items-center gap-2 text-foreground font-semibold">
				<div class="bg-foreground text-background rounded-full p-1">
					<Terminal class="h-3 w-3" />
				</div>
			</a>
			<nav
				class="flex flex-wrap justify-center gap-4 text-sm text-muted-foreground"
			>
				<a href="/" class="hover:text-foreground transition-colors">Home</a>
				<a href="/" class="hover:text-foreground transition-colors">Docs</a>
				<a
					href="https://github.com/SirBlobby/Clickploy"
					target="_blank"
					rel="noreferrer"
					class="hover:text-foreground transition-colors">GitHub</a
				>
				<a href="/" class="hover:text-foreground transition-colors">Support</a>
				<a href="/" class="hover:text-foreground transition-colors">Legal</a>
			</nav>
		</div>

		<div class="h-4 w-px bg-border hidden md:block"></div>

		<div class="flex items-center gap-2 text-sm font-medium">
			<div
				class={`h-2.5 w-2.5 rounded-full ${isNormal ? "bg-blue-500" : "bg-red-500"} animate-pulse`}
			></div>
			<span class={isNormal ? "text-blue-500" : "text-red-500"}>
				{isNormal ? "All systems normal." : status}
			</span>
		</div>
	</div>
</footer>
