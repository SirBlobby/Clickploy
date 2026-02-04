import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface User {
    ID: number;
    email: string;
    name: string;
    avatar: string;
}

const storedUser = browser ? localStorage.getItem('user') : null;
const storedToken = browser ? localStorage.getItem('token') : null;

export const user = writable<User | null>(storedUser ? JSON.parse(storedUser) : null);
export const token = writable<string | null>(storedToken);

export function login(userData: User, apiToken: string) {
    user.set(userData);
    token.set(apiToken);
    if (browser) {
        localStorage.setItem('user', JSON.stringify(userData));
        localStorage.setItem('token', apiToken);
    }
}

export function logout() {
    user.set(null);
    token.set(null);
    if (browser) {
        localStorage.removeItem('user');
        localStorage.removeItem('token');
    }
}
