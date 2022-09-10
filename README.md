# SecretNetwork-upgradable-contract

Recently, I've developed "upgradable" secret contract for SecretNetwork.

The documents says some methods( https://docs.scrt.network/secret-network-documentation/development/secret-contracts/best-practices/upgradability ) but these remains some problems.

I try to develop storage-contract pattern inspired by EVM's delegate call, this seems to work well.
This method may be well-knowned, but I cannot found any documents about this approach.
So I'll share it with community.
If you find any privacy or other risk in this solution, please inform me.

# Structure

I create two contract for upgradable:
  - 1: Storage contract which supply generic storage apis.
  - 2: Application contract which implements main contract logic.

Application contract does not uses it's own storage for application data store, but use storage contract by using inter-contract query or execute calls.
When upgrade application logic, simply deploy new application contract and set it uses same storage contract the old application contract uses.
This architecture is just a revers of EVM's upgradable contract pattern( https://docs.openzeppelin.com/learn/upgrading-smart-contracts#how-upgrades-work ).


# Security/Privacy Consideration
Without restriction, the storage contract can be accessed by any user or contract.
This may be security risk.

## Restrict caller contract
One simple solution is to restrict caller contract address so that only appropriate application can access storage.
It works well if you trust storage contract manage who can set trusted application contract address.
If the manager is malisious, he can write unrestricted contract which can read/write data from storage contract and set it is trusted.

## the "Permit" token

The secret-toolkit has a package named "permit"( https://github.com/scrtlabs/secret-toolkit/tree/master/packages/permit ), which is a simple signed document by user and contract is sure that the caller is a user himself by verifing its signature.

The permit implementation is not a generic authentication token but just for SNIP-20 or SNIP-721 tokens, but it can be used for test use.
For production use, I recommend to implement for token to have more secure data, for example expiration time.

# Contributes
Not just a PR of fixsing code, but any indication about literature is also welcome.
I am not good at English ;-)




