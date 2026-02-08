import { getProject, redeployProject, stopProject, type Project, type Deployment } from "$lib/api";
import { toast } from "svelte-sonner";

export class ProjectState {
	project = $state<Project | null>(null);
	loading = $state(true);
	activeDeploymentId = $state<string | null>(null);
	
	// We keep a string copy of logs for clipboard and initial loading, 
	// though xterm manages its own buffer.
	activeDeploymentLogs = $state(""); 
	
	ws: WebSocket | null = null;
	onLogData: ((data: string) => void) | null = null;
	onLogClear: (() => void) | null = null;

	latestDeployment = $derived(this.project?.deployments?.[0]);
	status = $derived(this.latestDeployment?.status || "unknown");

	constructor(public projectId: string) {}

	async init() {
		await this.loadProject();
		this.startStatusPoll();
	}

	private pollInterval: number | null = null;

	startStatusPoll() {
		if (typeof window === "undefined") return;
		if (this.pollInterval) window.clearInterval(this.pollInterval);
		this.pollInterval = window.setInterval(() => {
			if (this.status === "building") {
				this.loadProject();
			}
		}, 2000);
	}

	async loadProject() {
		if (!this.projectId) return;
		const res = await getProject(this.projectId);
		if (res) {
			this.project = res;
			const active = res.deployments?.find((d) => d.id === this.activeDeploymentId) ?? res.deployments?.[0];
			if (active) {
				this.selectDeployment(active, true);
			}
		}
		this.loading = false;
	}

	async refresh() {
		this.loading = true;
		await this.loadProject();
		this.loading = false;
	}

	async handleRedeploy() {
		if (!this.project) return;
		toast.info("Starting redeployment...");
		const success = await redeployProject(this.project.id);
		if (success) {
			toast.success("Redeployment started!");
			setTimeout(() => this.loadProject(), 1000);
		}
	}

	async handleStop() {
		if (!this.project) return;
		toast.info("Stopping project...");
		const success = await stopProject(this.project.id);
		if (success) {
			setTimeout(() => this.loadProject(), 1000);
		}
	}

	selectDeployment(deployment: Deployment, force = false) {
		if (this.activeDeploymentId === deployment.id && !force) return;

		this.activeDeploymentId = deployment.id;
		this.activeDeploymentLogs = deployment.logs || "";

		// Reset xterm
		if (this.onLogClear) this.onLogClear();
		
		// Write existing logs
		// We format CR LF for xterm
		const formattedLogs = (deployment.logs || "").replace(/\n/g, "\r\n");
		if (this.onLogData) this.onLogData(formattedLogs);

		if (deployment.status === "building") {
			this.connectWebSocket(deployment.id);
		} else {
			this.closeWebSocket();
		}
	}

	handleBuildCompleted() {
		// When a build finishes, refresh project data to update status/UI.
		this.loadProject();
	}

	connectWebSocket(deploymentId: string) {
		this.closeWebSocket();
		// Ensure we are in browser
		if (typeof window === "undefined") return;

		const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
		this.ws = new WebSocket(
			`${protocol}//${window.location.hostname}:8080/api/deployments/${deploymentId}/logs/stream`,
		);

		this.ws.onmessage = (event) => {
			this.activeDeploymentLogs += event.data;
			// Pass raw data to xterm, it handles ANSI codes. 
			// Ensure newlines are treated as CRLF for terminal
			const chunk = event.data.replace(/\n/g, "\r\n");
			if (this.onLogData) {
				this.onLogData(chunk);
			}
		};

		this.ws.onclose = () => {
			console.log("Log stream closed");
			// If we were building, pull latest status once stream ends.
			if (this.status === "building") {
				this.loadProject();
			}
		};
	}

	closeWebSocket() {
		if (this.ws) {
			this.ws.close();
			this.ws = null;
		}
	}

	copyLogs() {
		if (typeof navigator !== "undefined") {
			navigator.clipboard.writeText(this.activeDeploymentLogs);
			toast.success("Logs copied to clipboard");
			return true;
		}
		return false;
	}

	destroy() {
		this.closeWebSocket();
		if (this.pollInterval) {
			window.clearInterval(this.pollInterval);
			this.pollInterval = null;
		}
	}
}
