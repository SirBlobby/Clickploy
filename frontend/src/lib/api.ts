import { toast } from "svelte-sonner";
import { get } from 'svelte/store';
import { token } from './auth';

const API_BASE = "http://localhost:8080";

export interface DeployResponse {
  status: string;
  app_name: string;
  port: number;
  url: string;
  message: string;
}

export interface AuthResponse {
	token: string;
	user: any;
}

export interface EnvVar {
	key: string;
	value: string;
}

export interface Deployment {
	id: string;
	project_id: string;
	status: string;
	commit: string;
	logs: string;
	url: string;
	created_at: string;
	updated_at: string;
}

export interface Project {
	id: string;
	name: string;
	repo_url: string;
	port: number;
	deployments: Deployment[];
	env_vars: EnvVar[];
	webhook_secret: string;
	git_token?: string;
	runtime?: string;
}

export async function getProject(id: string): Promise<Project | null> {
	try {
		return await fetchWithAuth(`/api/projects/${id}`);
	} catch (e: any) {
		console.error(e);
		return null;
	}
}

async function fetchWithAuth(url: string, options: RequestInit = {}) {
	const t = get(token);
	const headers = {
		"Content-Type": "application/json",
		...options.headers,
		...(t ? { Authorization: `Bearer ${t}` } : {}),
	};

	const res = await fetch(`${API_BASE}${url}`, { ...options, headers });
	if (!res.ok) {
		const err = await res.json();
		throw new Error(err.error || "Request failed");
	}
	return res.json();
}

export async function loginUser(email: string, password: string): Promise<AuthResponse | null> {
	try {
		return await fetchWithAuth("/auth/login", {
			method: "POST",
			body: JSON.stringify({ email, password }),
		});
	} catch (e: any) {
		toast.error(e.message);
		return null;
	}
}

export async function registerUser(email: string, password: string, name: string): Promise<AuthResponse | null> {
	try {
		return await fetchWithAuth("/auth/register", {
			method: "POST",
			body: JSON.stringify({ email, password, name }),
		});
	} catch (e: any) {
		toast.error(e.message);
		return null;
	}
}

export async function createProject(
	repo: string,
	name: string,
	port?: number,
	envVars?: Record<string, string>,
	gitToken?: string,
	buildCommand?: string,
	startCommand?: string,
	installCommand?: string,
	runtime?: string
): Promise<Project | null> {
	try {
		return await fetchWithAuth("/api/projects", {
			method: "POST",
			body: JSON.stringify({
				repo,
				name,
				port,
				env_vars: envVars,
				git_token: gitToken,
				build_command: buildCommand,
				start_command: startCommand,
				install_command: installCommand,
				runtime: runtime,
			}),
		});
	} catch (e: any) {
		toast.error(e.message);
		return null;
	}
}

export async function updateProjectEnv(id: string, envVars: Record<string, string>): Promise<boolean> {
	try {
		await fetchWithAuth(`/api/projects/${id}/env`, {
			method: "PUT",
			body: JSON.stringify({ env_vars: envVars }),
		});
		return true;
	} catch (e: any) {
		toast.error(e.message);
		return false;
	}
}

export async function redeployProject(id: string): Promise<boolean> {
	try {
		await fetchWithAuth(`/api/projects/${id}/redeploy`, {
			method: "POST",
		});
		return true;
	} catch (e: any) {
		toast.error(e.message);
		return false;
	}
}

export async function listProjects(): Promise<Project[] | null> {
	try {
		return await fetchWithAuth("/api/projects");
	} catch (e: any) {
		console.error(e);
		return null;
	}
}

export async function deployApp(repo: string, name: string, port?: number): Promise<DeployResponse | null> {
	return createProject(repo, name, port) as any;
}

export async function listActivity(): Promise<any[] | null> {
	try {
		return await fetchWithAuth("/api/activity");
	} catch (e: any) {
		console.error(e);
		return null;
	}
}

export async function updateProfile(name: string, email: string) {
	try {
		return await fetchWithAuth("/api/user/profile", {
			method: "PUT",
			body: JSON.stringify({ name, email }),
		});
	} catch (e: any) {
		toast.error(e.message);
		return null;
	}
}

export async function getSystemStatus() {
	try {
		const res = await fetch(`${API_BASE}/api/system/status`);
		if (res.ok) {
			return await res.json();
		}
		return null;
	} catch (e: any) {
		console.error(e);
		return null;
	}
}

export async function updatePassword(oldPassword: string, newPassword: string) {
	try {
		return await fetchWithAuth("/api/user/password", {
			method: "PUT",
			body: JSON.stringify({ old_password: oldPassword, new_password: newPassword }),
		});
	} catch (e: any) {
		toast.error(e.message);
		return null;
	}
}

export async function getStorageStats() {
	try {
		return await fetchWithAuth("/api/storage/stats");
	} catch (e: any) {
		console.error(e);
		return null;
	}
}

export async function listDatabases() {
	try {
		return await fetchWithAuth("/api/storage/databases");
	} catch (e: any) {
		console.error(e);
		return [];
	}
}

export async function createDatabase(name: string, type: string = "sqlite") {
	try {
		return await fetchWithAuth("/api/storage/databases", {
			method: "POST",
			body: JSON.stringify({ name, type }),
		});
	} catch (e: any) {
		toast.error(e.message);
		return null;
	}
}

export async function getAdminUsers() {
	try {
		return await fetchWithAuth("/api/admin/users");
	} catch (e: any) {
		toast.error(e.message);
		return [];
	}
}

export async function deleteAdminUser(id: string) {
	try {
		await fetchWithAuth(`/api/admin/users/${id}`, {
			method: "DELETE",
		});
		return true;
	} catch (e: any) {
		toast.error(e.message);
		return false;
	}
}

export async function getAdminStats() {
	try {
		return await fetchWithAuth("/api/admin/stats");
	} catch (e: any) {
		console.error(e);
		return null;
	}
}
