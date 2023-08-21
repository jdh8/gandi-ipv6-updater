import assert from "node:assert/strict";
import fetch from "node-fetch";
import { networkInterfaces } from "os";
import "dotenv/config";

const ipv6 = Object.values(networkInterfaces()).flat()
	.find(entry => entry.scopeid === 0 && !entry.internal)
	.address;

const { APIKEY, FQDN, NAME } = process.env;
const uri = `https://api.gandi.net/v5/livedns/domains/${FQDN}/records/${NAME}/AAAA`;

assert((await fetch(uri, {
	method: "PUT",
	headers: {
		Authorization: `Apikey ${APIKEY}`,
		"Content-Type": "application/json",
	},
	body: `{"rrset_values":["${ipv6}"]}`,
})).ok);
