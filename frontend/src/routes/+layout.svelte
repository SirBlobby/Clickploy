<script lang="ts">
	import "./layout.css";
	import favicon from "$lib/assets/favicon.svg";
	import { Toaster } from "svelte-sonner";
	import Navbar from "$lib/components/Navbar.svelte";
	import Footer from "$lib/components/Footer.svelte";

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="h-screen flex flex-col overflow-hidden bg-background text-foreground">
	<Navbar />
	<Toaster position="top-right" richColors />
	<div class="flex-1 flex flex-col min-h-0 overflow-hidden">
		<main class="flex-1 flex flex-col min-h-0 overflow-y-auto">
			{@render children()}
		</main>
		<!-- <Footer /> -->
	</div>
</div>
