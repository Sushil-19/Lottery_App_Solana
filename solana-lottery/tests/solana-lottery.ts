import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolanaLottery } from '../target/types/solana_lottery';

describe('solana-lottery', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaLottery as Program<SolanaLottery>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
