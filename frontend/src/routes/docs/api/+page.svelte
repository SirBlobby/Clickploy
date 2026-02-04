<script lang="ts">
	import { Separator } from "$lib/components/ui/separator";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
	} from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import {
		Table,
		TableBody,
		TableCell,
		TableHead,
		TableHeader,
		TableRow,
	} from "$lib/components/ui/table";
</script>

<div class="space-y-6">
	<div class="space-y-2">
		<h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">
			API Reference
		</h1>
		<p class="text-xl text-muted-foreground">
			The backend exposes a RESTful API for automation and integration. All
			authenticated endpoints require a <code>Bearer</code> token.
		</p>
	</div>
	<Separator />

	<div class="space-y-8">
		<!-- Project Endpoints -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">Projects</h3>
			<Card>
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead class="w-[100px]">Method</TableHead>
							<TableHead>Endpoint</TableHead>
							<TableHead>Description</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						<TableRow>
							<TableCell><Badge>GET</Badge></TableCell>
							<TableCell class="font-mono">/api/projects</TableCell>
							<TableCell>List all projects for the current user.</TableCell>
						</TableRow>
						<TableRow>
							<TableCell><Badge>POST</Badge></TableCell>
							<TableCell class="font-mono">/api/projects</TableCell>
							<TableCell>Create and deploy a new project.</TableCell>
						</TableRow>
						<TableRow>
							<TableCell><Badge variant="secondary">GET</Badge></TableCell>
							<TableCell class="font-mono">/api/projects/:id</TableCell>
							<TableCell>Get details of a specific project.</TableCell>
						</TableRow>
						<TableRow>
							<TableCell><Badge>POST</Badge></TableCell>
							<TableCell class="font-mono">/api/projects/:id/redeploy</TableCell>
							<TableCell>Trigger a manual redeployment.</TableCell>
						</TableRow>
						<TableRow>
							<TableCell><Badge variant="outline">PUT</Badge></TableCell>
							<TableCell class="font-mono">/api/projects/:id/env</TableCell>
							<TableCell>Update environment variables.</TableCell>
						</TableRow>
					</TableBody>
				</Table>
			</Card>
		</div>

		<!-- Activity & Stats -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">Activity & Data</h3>
			<Card>
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead class="w-[100px]">Method</TableHead>
							<TableHead>Endpoint</TableHead>
							<TableHead>Description</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						<TableRow>
							<TableCell><Badge variant="secondary">GET</Badge></TableCell>
							<TableCell class="font-mono">/api/activity</TableCell>
							<TableCell>Get recent deployment activity.</TableCell>
						</TableRow>
					</TableBody>
				</Table>
			</Card>
		</div>

		<!-- System Endpoints -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">System & Admin</h3>
			<Card>
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead class="w-[100px]">Method</TableHead>
							<TableHead>Endpoint</TableHead>
							<TableHead>Description</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						<TableRow>
							<TableCell><Badge variant="secondary">GET</Badge></TableCell>
							<TableCell class="font-mono">/api/system/status</TableCell>
							<TableCell>Check backend health and version.</TableCell>
						</TableRow>
						<TableRow>
							<TableCell><Badge variant="secondary">GET</Badge></TableCell>
							<TableCell class="font-mono">/api/admin/stats</TableCell>
							<TableCell>Get global system statistics (Admin only).</TableCell>
						</TableRow>
						<TableRow>
							<TableCell><Badge variant="secondary">GET</Badge></TableCell>
							<TableCell class="font-mono">/api/admin/users</TableCell>
							<TableCell>List all users (Admin only).</TableCell>
						</TableRow>
					</TableBody>
				</Table>
			</Card>
		</div>
	</div>
</div>
