import { assert, expect, test } from 'vitest'
import '@fluencelabs/js-client.node';
import { Fluence } from "@fluencelabs/js-client.api";

import { get_logs, test_join, inspect, test as test_connectivity } from "../src/remove_worker";

import { exec } from "child_process";

// Edit an assertion and save to see HMR in action

const RELAY = "/ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWJDiLFLmWstcFpAofWkYJzuvwuquNTQQkB9xzKjRyqqFJ";

test('Init Fluence Peer', async () => {
  await Fluence.connect(RELAY);
})

test('get logs', async () => {
  await Fluence.connect(RELAY);
  let logs = await get_logs();
})

test('run fluence cli --help', () => new Promise<void>(async done => {
  exec(`./hack.sh`, (error, stdout, stderr) => {
    if (error) {
        console.log(`error: ${error.message}`);
    }
    if (stderr) {
        console.log(`stderr: ${stderr}`);
    }
    console.log(`stdout: ${stdout}`);

    done();
  });
}))

test('run fluence cli run inspect()', () => new Promise<void>(done => {
  exec("npx fluence run -f 'inspect()' -i ../../aqua/remove_worker.aqua", (error, stdout, stderr) => {
    if (error) {
        console.log(`error: ${error.message}`);
    }
    if (stderr) {
        console.log(`stderr: ${stderr}`);
    }
    console.log(`stdout: ${stdout}`);

    done();
  });
}))

type Output = {
  error: Error | null
  stderr: string
  stdout: string
};

async function runCli(command: string): Promise<Output> {
  return new Promise<Output>(done =>
    exec(command, (error, stdout, stderr) => {
      done({
        error,
        stderr,
        stdout
      })
    })
  );
}

test('deploy & match a single deal', async () => {
  let providerKey = "0xbb3457514f768615c8bc4061c7e47f817c8a570c5c3537479639d4fad052a98a";
  let registerProvider = await runCli(
    `npx fluence compute-provider matching registration 1 --privKey ${providerKey} --network local`
  );
  assert(registerProvider.error === null, `Error happened on matching: ${registerProvider.error}`);

  let devKey = "0xbb3457514f768615c8bc4061c7e47f817c8a570c5c3537479639d4fad052a98a";
  let deployDeal = await runCli(`deal deploy --privKey ${devKey} --network local`);
  assert(deployDeal.error === null, `Error happened on deploying deal: ${deployDeal}`);

  console.dir(deployDeal);
})

// Test on deployment
// 1. deploy deal
// 2. match
// 3. test it's in Joined Deals on the decider
// 4. test worker exists on the host
// 5. test Srv.list on worker returns expected services
//
// Test on multiple deals deployment
// 1. deploy several deals
// 2. match all of them
// 3. test all of them are in Joined Deal on the decider
// 4. test all workers exist on the host
// 5. test Srv.list is correct for all workers
