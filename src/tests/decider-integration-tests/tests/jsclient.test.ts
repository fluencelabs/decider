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

test('run fluence cli --help', () => new Promise<void>(done => {
  exec("npx fluence ", (error, stdout, stderr) => {
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

test('run fluence cli run', () => new Promise<void>(done => {
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
