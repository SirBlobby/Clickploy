<script lang="ts">
	import "./layout.css";
	import favicon from "$lib/assets/favicon.svg";
	import { Toaster } from "svelte-sonner";
	import Navbar from "$lib/components/Navbar.svelte";
	import Footer from "$lib/components/Footer.svelte";

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Navbar />
<Toaster position="top-right" richColors />
<div class="min-h-screen bg-background text-foreground flex flex-col">
	<div class="flex-1">
		{@render children()}
	</div>
	<Footer />
</div>
