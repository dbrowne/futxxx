# FE Challenge

# Thoughts in general before providing thoughts on the design
0. Disclaimer. My vocabulary might not accurately represent the appropriate domain knowledge but I'm trying here. 
1. In distributed systems one of the key concepts is "Byzantyne Fault Tolerance" described in Lamports paper ["The Byzantyne General's Problem"](https://dl.acm.org/doi/10.1145/357172.357176)
2. In systems designs(where there is no incentive to be faulty ) the minimum number of nodes necessary to tolerate f faullty nodes would be 3xf+1.
3. So if we have n total nodes and f faulty nodes we have n-f nodes whith the constraint of n >= 3f+1 or n-f >= 2f+1 nodes  or ~ 2/3 of the nodes must not be faulty. (I hate hearing people telling me it is 51%. They never read the paper.)
4. The problem presents itself in line 2 where there is no incentive to be faulty or in the case of an oracle, presenting false information.
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



## development hindered due to local hw issues  Code coming later today (2 hours spent on hw issues now using burner laptop which does successfully run all Solana related programs)
# 🧩 Summary of Issue (AVX vs AVX2)

## 🔧 System
- **Machine:** HP Z820 Workstation
- **CPU:** Dual Intel Xeon E5-2667 v2 (Ivy Bridge, 2013)
- **CPU Features:**
   - ✅ AVX
   - ❌ AVX2

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


# skeleton code:

