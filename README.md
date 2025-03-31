# FE Challenge

# Thoughts in general before providing thoughts on the design
0. Disclaimer. My vocabulary might not accurately represent the appropriate domain knowledge but I'm trying here. 
1. In distributed systems one of the key concepts is "Byzantyne Fault Tolerance" described in Lamports paper ["The Byzantyne General's Problem"](https://dl.acm.org/doi/10.1145/357172.357176)
2. In systems designs(where there is no incentive to be faulty ) the minimum number of nodes necessary to tolerate f faullty nodes would be 3xf+1.
3. So if we have n total nodes and f faulty nodes we have n-f nodes whith the constraint of n >= 3f+1 or n-f >= 2f+1 nodes  or ~ 2/3 of the nodes must not be faulty. (I hate hearing people telling me it is 51%. They never read the paper.)
4. The problem presents itself in line 2 where there is incentive to be faulty or in the case of an oracle, presenting false information.
5. In the case of monetized Oracles, dishonest behavior can be profitable so we now have to address at least three concepts to mitigate an intentionally dishonest majority:   
   1. Distributed systems 
   2. Game theory (which is not my strength but I can talk about the prisoner's dilemma ~ Nash equilibrium)
   3. cryptography (can talk about encryption but ZKP is pretty cool)
6. Without discussing ii and ii and diving into the design, we can look at Oracle manipulation as a modern version of the general's problem where 
   1. honest general , traitorous general  vs.  honest Oracle , malicious Oracle
   2. traitors, false messages vs.  malicious oracles, early release/lies
   3. consensus on the decision  vs.   orcale consensus of true/false
   4. intercepted messages vs.  collusion to influence reveals.


# Thoughts on the design:
## Strengths
1. Mutually assured honesty by slashing colluding oracles.
2. Simple validation with boolean output simplifies the design.
3. Monetary requirements to join the network prevent the creation of infinite oracles/participants.
4. Truth protection via commit-reveal prevents voting influences. 


## Weaknesses
1. Slash incetivization. An honest oracle might hesitate to slash dishonest peers for fear of retaliation or collusion in the future.
2. Out of band (off chain) coordination is possible and difficult,  if not impossible to detect.
3. No way to detect collusion if there is no proof of communication between oracles. eg the need to run in a TEE. 
4. Potential for abuse since slashing rewards might incetivise others to coerce oracles into punishable behavior. 
5. Potential for oracles to not reveal if they determine the vote was incorrect even if it is committed. (seems to be the case but unsure)
6. How can we determine when an oracle is resolved? Is there a mandatory reveal time?

## Potential improvements
 1. Have reliability stats on oracles to build a reputation layer.
2. Create an enforced time reveal window
3. create a quorum: eg. resolution can only occur after a minimum number of Oracles have voted. 




## FYI issue resolved but.. development was hindered due to local hw issues.  Code coming later today (2 hours spent on hw issues.  now using burner laptop which does successfully run all Solana related programs)
# 🧩 Summary of Issue (AVX vs AVX2)

## 🔧 System
- **Machine:** HP Z820 Workstation 512GB ram (was great for simulating virtualized kubernetes clusters)
- **CPU:** Dual Intel Xeon E5-2667 v2 (Ivy Bridge, 2013)
- **CPU Features:**
   - ✅ AVX
   - ❌ [AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions)  Needed for Solana development

---

## 🎯 Goal
- Run `solana-test-validator` (Solana v1.14.17)
- Use `anchor-cli 0.28.0` (which is compatible with Solana 1.14.x)
- Build everything locally to avoid crashing on AVX2 instructions

---

## 💥 The Problem (also lack of experience and domain knowledge)
Even after trying to compile Solana from source with the following:

```toml
# .cargo/config.toml
[build]
rustflags = ["-C", "target-cpu=core-avx-i", "-C", "target-feature=-avx2"]

```
## 💥 Docker images also fail since they are built with AVX2 support  (not a biggie but irksome)


# Approach:
## Based on what I saw and the example programs My anchor program will have the following components:
1. Oracle for the event being resolved
2. Commitment to store the hashed vote
3. User collateral

## Contract logic:
1. Create_network:  create a network with a USDC collateral requirement
2. Join_network:  
3. commit : Save the hash (bit, salt) as the immutable commitment
4. reveal: check the (bit,salt) and mark it revealed
5. resolve:  store the resolution
6. slash:   slashing mechanism.



## 💥 Build Problems  (mega time suck!! 90 minutes++) : 
0. Have to manually change the cargo lock version to 3 due to anchor/solana/rust version compatiblity issues
1.   looks like cargo-build-sbf has a dependency on a bundled cargo version that does not understand cargo lock v 4
2. Had to incorporate a local version of ahash to address the following build error
```
error[E0658]: use of unstable library feature 'build_hasher_simple_hash_one'
--> src/random_state.rs:463:5
|
463 | /     fn hash_one<T: Hash>(&self, x: T) -> u64 {
464 | |         RandomState::hash_one(self, x)
465 | |     }
| |_____^
|
```
3. Have wallet issues  so only testing on localnet:

## test results
1. basic test passes
```

djb@SGX:~/code/futxxx/oracle-code$  git:(main) 5A 6M 8Aanchor build
anchor deploy --provider.cluster localnet
warning: unused config key `unstable.lockfile-version` in `/home/djb/code/futxxx/oracle-code/.cargo/config.toml`
    Finished release [optimized] target(s) in 0.24s
Deploying cluster: http://localhost:8899
Upgrade authority: /home/djb/.config/solana/id.json
Deploying program "oracle"...
Program path: /home/djb/code/futxxx/oracle-code/target/deploy/oracle.so...
Program Id: Bs7GGMzNW9nhrZrhxyaLW1AQiaQX6kk1CTqfvj1RkRvS

Deploy success
djb@SGX:~/code/futxxx/oracle-code$  git:(main) 5A 6M 8AANCHOR_PROVIDER_URL=http://127.0.0.1:8899 \
ANCHOR_WALLET=target/deploy/oracle-keypair.json \
npx mocha tests/


  oracle
Test runs
    ✔ Is initialized!


  1 passing (3ms)

djb@SGX:~/code/futxxx/oracle-code$  git:(main) 5A 6M 8A


```

## 💥 Need some hints  as I am in npm hell and cannot get the tests to work.


## Created .js test. (ran ok) Oracle.ts test does not run.  


## will proceed with the UI once I can verify my tests (testing on  localnet)
