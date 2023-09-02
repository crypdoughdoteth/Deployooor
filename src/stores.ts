import { writable } from "svelte/store";
export let configuration = writable<Config>({ provider: '', keystore: '' });
export let deployment = writable<deploymentDetails>(
    {
        sc_name: '',
        deployer_address: '',
        deploy_date: '',
        sc_address: '',
        network: ''
    });

export type Config = {
    provider: string;
    keystore: string;
};

export type deploymentDetails = {
    sc_name: string;
    deployer_address: string;
    deploy_date: string;
    sc_address: string;
    network: string;
}