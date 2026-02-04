<script lang="ts">
    import { onMount } from "svelte";
    import { getProfile, regenerateAPIKey } from "$lib/api";
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import {
        Card,
        CardContent,
        CardHeader,
        CardTitle,
        CardDescription,
        CardFooter,
    } from "$lib/components/ui/card";
    import { Loader2, Copy, RefreshCw, Terminal } from "@lucide/svelte";
    import { toast } from "svelte-sonner";

    let loading = $state(true);
    let apiKey = $state("");
    let regenerating = $state(false);

    onMount(async () => {
        const profile = await getProfile();
        if (profile) {
            apiKey = profile.api_key;
        }
        loading = false;
    });

    async function handleRegenerate() {
        if (
            !confirm("Are you sure? This will invalidate your current API key.")
        )
            return;

        regenerating = true;
        const res = await regenerateAPIKey();
        regenerating = false;

        if (res && res.api_key) {
            apiKey = res.api_key;
            toast.success("API Key regenerated successfully");
        }
    }

    function copyKey() {
        navigator.clipboard.writeText(apiKey);
        toast.success("API Key copied to clipboard");
    }
</script>

<div class="container mx-auto py-10 px-4 max-w-4xl">
    <div class="space-y-6">
        <div>
            <h2 class="text-3xl font-bold tracking-tight">
                Session & API Access
            </h2>
            <p class="text-muted-foreground">
                Manage your API keys for CLI and external access.
            </p>
        </div>

        {#if loading}
            <div class="flex justify-center p-10">
                <Loader2 class="h-8 w-8 animate-spin" />
            </div>
        {:else}
            <Card class="border-border/60">
                <CardHeader>
                    <CardTitle>Personal Access Token</CardTitle>
                    <CardDescription>
                        Use this token to authenticate with the Clickploy CLI
                        and API. Treat this token like a password.
                    </CardDescription>
                </CardHeader>
                <CardContent class="space-y-4">
                    <div class="space-y-2">
                        <Label>Your API Key</Label>
                        <div class="flex items-center gap-2">
                            <Input
                                readonly
                                value={apiKey}
                                class="font-mono bg-muted"
                                type="password"
                            />
                            <Button
                                variant="outline"
                                size="icon"
                                onclick={copyKey}
                            >
                                <Copy class="h-4 w-4" />
                            </Button>
                        </div>
                    </div>

                    <div
                        class="bg-muted/50 p-4 rounded-lg space-y-2 border border-border/50"
                    >
                        <div
                            class="flex items-center gap-2 text-sm font-medium"
                        >
                            <Terminal class="h-4 w-4" />
                            CLI Usage
                        </div>
                        <div
                            class="text-xs font-mono bg-background p-2 rounded border border-border/50"
                        >
                            clickploy login --token {apiKey}
                        </div>
                        <p class="text-xs text-muted-foreground">
                            Or use it in the Authorization header for API
                            requests:
                        </p>
                        <div
                            class="text-xs font-mono bg-background p-2 rounded border border-border/50"
                        >
                            Authorization: Bearer {apiKey}
                        </div>
                    </div>
                </CardContent>
                <CardFooter
                    class="border-t px-6 py-4 flex justify-between items-center bg-muted/40"
                >
                    <span class="text-xs text-muted-foreground">
                        Last generated: {new Date().toLocaleDateString()}
                    </span>
                    <Button
                        variant="secondary"
                        size="sm"
                        onclick={handleRegenerate}
                        disabled={regenerating}
                    >
                        {#if regenerating}
                            <Loader2 class="h-3 w-3 mr-2 animate-spin" />
                        {:else}
                            <RefreshCw class="h-3 w-3 mr-2" />
                        {/if}
                        Regenerate Token
                    </Button>
                </CardFooter>
            </Card>
        {/if}
    </div>
</div>
