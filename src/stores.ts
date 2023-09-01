import { writable } from "svelte/store";
export let configuration = writable<Config>({ provider: '', keystore: '' });
export let deployment = writable<deploymentDetails>(
    {
        contractName: '',
        deployerAddress: '',
        date: '',
        contractAddress: '',
        network: ''
    });

export type Config = {
    provider: string;
    keystore: string;
};

export type deploymentDetails = {
    contractName: string;
    deployerAddress: string;
    date: string;
    contractAddress: string;
    network: string;
}