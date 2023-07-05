import { writable } from "svelte/store";

export const taskList = writable([]);
export const web3taskList = writable([]);

export const userAddress = writable(undefined);

export const darkTheme = writable(false);

export const walletConnected = writable(false);
