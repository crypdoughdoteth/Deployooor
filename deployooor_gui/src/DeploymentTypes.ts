export interface ContractDeployment {
  provider: string;
  args?: string[];
  path: string;
  private_key: string;
}

export interface NetworkSettings {
  name: string;
  provider: string;
  etherscan_api: string | null;
}
