import { exec } from "child_process";

async function run(command) {
  return new Promise(done =>
    exec(command, (error, stdout, stderr) => {
      done({
        error,
        stderr,
        stdout
      })
    })
  );
}

// console.dir(await run("fluence --help"))

function assert(condition, message) {
  if (!condition) {
    throw message
  }
}

async function cleanup() {
  let rm = await run('rm -rf $(pwd)/tests/sample_project/.fluence');
  assert(rm.error === null, `Failed to remove .fluence in sample_project: ${rm.error}`);
  console.log("removed .fluence dir");
}

async function deployMatch() {
  await cleanup();

  let providerKey = "0xbb3457514f768615c8bc4061c7e47f817c8a570c5c3537479639d4fad052a98a";
  let registerProvider = await run(
    `./hack.sh compute-provider matching registration 1 --privKey ${providerKey} --network local`
  );
  assert(registerProvider.error === null, `Error happened on matching: ${registerProvider.error}`);
  console.log("registered provider");

  let devKey = "0x1a1bf9026a097f33ce1a51f5aa0c4102e4a1432c757d922200ef37df168ae504";
  let deployDeal = await run(`./hack.sh deal deploy --privKey ${devKey} --network local`);
  assert(deployDeal.error === null, `Error happened on deploying deal: ${deployDeal.error}`);

  let line = deployDeal.stdout.split('\n').find(line => line.includes('https://explorer.testnet.aurora.dev/address'));
  let deal = line.split('https://explorer.testnet.aurora.dev/address/')[1];

  console.log(`deployed deal ${deal}`);

  let matcherKey = "0xcb448613322f0ae09bb111e6bfd5be93480f1ec521b062a614f9af025c8f1852";
  let match = await run(`./hack.sh deal match --privKey ${matcherKey} --network local ${deal}`);
  assert(match.error === null, `Error happened on matching deal: ${match.error}`);
  let numWorkersLine = match.stdout.split('\n').find(line => line.includes(`workers joined to deal ${deal}`));
  let numWorkers = numWorkersLine.split('workers joined to deal')[0].trim();
  assert(numWorkers == "1", `expected 1 worker, got ${numWorkers}`);

  console.log(`matched ${numWorkers} workers`);
}

await deployMatch();
