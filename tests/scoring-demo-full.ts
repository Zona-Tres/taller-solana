import * as anchor from '@coral-xyz/anchor';
import {Program, web3} from '@coral-xyz/anchor';
import {ScoringDemoFull} from '../target/types/scoring_demo_full';

describe('scoring-demo-full', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    // Define our program.
    const program = anchor.workspace.ScoringDemoFull as Program<ScoringDemoFull>;

    // Define element to score.
    const elementToScore = 'airport1::bathroom1';

    // Generate voter keypair.
    const voter = anchor.web3.Keypair.generate();

    it('Open scoring', async () => {
        // Generate creator keypair.
        const creator = anchor.web3.Keypair.generate();

        // Derive scoring account pubkey.
        const [scoringAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(elementToScore)],
            program.programId);

        // Build the transaction.
        const tx = await program.methods.openScoring(elementToScore).accounts({
            scoringAccount,
            creator: creator.publicKey,
            payer: anchor.Wallet.local().payer.publicKey,
        }).signers([creator]).rpc();

        console.log('Your transaction signature', tx);

        const scoringAccountData = await program.account.scoringAccount.fetch(scoringAccount);
        console.log('Scoring account', JSON.stringify(scoringAccountData, null, 2));
    });

    it('Vote', async () => {

        // Derive scoring account pubkey.
        const [scoringAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(elementToScore)],
            program.programId);

        // Derive vote account pubkey.
        const [voteAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [scoringAccount.toBuffer(), voter.publicKey.toBuffer()], program.programId);

        // Build the transaction.
        const tx = await program.methods.vote({
            high: null,
        }).accounts({
            scoringAccount,
            voteAccount,
            voter: voter.publicKey,
            payer: anchor.Wallet.local().payer.publicKey,
        }).signers([voter]).rpc();

        console.log('Your transaction signature', tx);

        const scoringAccountData = await program.account.scoringAccount.fetch(scoringAccount);
        console.log('Scoring account', JSON.stringify(scoringAccountData, null, 2));

        const voteAccountData = await program.account.voteAccount.fetch(voteAccount);
        console.log('Vote account', JSON.stringify(voteAccountData, null, 2));
    });

    it('Edit Vote', async () => {
        // Derive scoring account pubkey.
        const [scoringAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(elementToScore)],
            program.programId);

        // Derive vote account pubkey.
        const [voteAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [scoringAccount.toBuffer(), voter.publicKey.toBuffer()], program.programId);

        // Build the transaction.
        const tx = await program.methods.editVote({
            low: null,
        }).accounts({
            scoringAccount,
            voteAccount,
            voter: voter.publicKey,
        }).signers([voter]).rpc();

        console.log('Your transaction signature', tx);

        const scoringAccountData = await program.account.scoringAccount.fetch(scoringAccount);
        console.log('Scoring account', JSON.stringify(scoringAccountData, null, 2));

        const voteAccountData = await program.account.voteAccount.fetch(voteAccount);
        console.log('Vote account', JSON.stringify(voteAccountData, null, 2));
    });

    it('Edit Vote - incorrect voter', async () => {
        // Derive scoring account pubkey.
        const [scoringAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(elementToScore)],
            program.programId);

        // Derive vote account pubkey.
        const [voteAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [scoringAccount.toBuffer(), voter.publicKey.toBuffer()], program.programId);

        // Build the transaction.
        try {
            await program.methods.editVote({
                low: null,
            }).accounts({
                scoringAccount,
                voteAccount,
                voter: web3.Keypair.generate().publicKey,
            }).signers([voter]).rpc();
        } catch (e) {
            // Ok
            return;
        }

        throw new Error('Expected error');
    });
});
