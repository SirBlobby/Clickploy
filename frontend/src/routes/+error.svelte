<script lang="ts">
	import { page } from '$app/stores';
	import { Button } from "$lib/components/ui/button";
	import { AlertTriangle, Home, ArrowLeft } from "@lucide/svelte";
</script>

<div class="flex flex-col items-center justify-center min-h-[70vh] text-center px-4 animate-in fade-in zoom-in duration-300">
	<div class="bg-destructive/10 p-6 rounded-full mb-6">
		<AlertTriangle class="h-12 w-12 text-destructive" />
	</div>
	
	<h1 class="text-4xl font-bold tracking-tight mb-2">
		{$page.status}
	</h1>
	
	<p class="text-xl text-muted-foreground mb-8 max-w-md text-balance">
		{$page.error?.message || "Something went wrong."}
	</p>

	<div class="flex gap-4">
		<Button variant="outline" onclick={() => history.back()}>
			<ArrowLeft class="mr-2 h-4 w-4" />
			Go Back
		</Button>
		<Button href="/">
			<Home class="mr-2 h-4 w-4" />
			Home
		</Button>
	</div>
</div>
