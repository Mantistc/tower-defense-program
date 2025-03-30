export * from "./initialize-player";
export * from "./update-player-game-values"
export * from "./update-player-authority"

export enum InstructionDiscriminator {
  InitializePlayer,
  UpdatePlayerGameValues,
  UpdatePlayerAuthority
}
