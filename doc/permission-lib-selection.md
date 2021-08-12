casbin  **PERM metamodel (Policy, Effect, Request, Matchers)**

oso     **Polar Language Reference**


Hot Update Permissions

  Single machine hot update permission `casbin` is a bit more flexible,
  Synchronization permission changes between clusters need to be implemented by users themselves.

  Single-machine hot update permissions `oso` also ok,
  Synchronization of permissions changes between clusters needs to be implemented by users themselves.


The definition of `casbin` is **simple and easy to change**.

`oso` is relatively **difficult to define, but more expressive**.
 
 If you do permissions based on casbin, it is probably the least mentally burdensome for the user using it and the least complicated to understand, **so priority is given to casbin**.

`oso` is actually a **very good permission authentication library**, and if I use it personally I am likely to use oso because it is **very expressive and flexible**, and I will introduce it into my other works in the future.

Or the **future may support a variety of authentication methods**, **the first step to achieve casbin authentication**, the **second step to complement the function of oso authentication**.