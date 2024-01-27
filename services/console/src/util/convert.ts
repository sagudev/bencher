import { PlanLevel } from "../types/bencher";

export const dateTimeMillis = (date_str: undefined | string) => {
	if (date_str === undefined) {
		return null;
	}
	const date_ms = Date.parse(date_str);
	if (date_ms) {
		const date = new Date(date_ms);
		if (date) {
			return date.getTime();
		}
	}
	return null;
};

export const fmtDate = (date_str: undefined | string) => {
	if (date_str === undefined) {
		return null;
	}
	const date_ms = Date.parse(date_str);
	if (date_ms) {
		const date = new Date(date_ms);
		if (date) {
			return date.toDateString();
		}
	}
	return null;
};

export const planLevel = (level: undefined | PlanLevel) => {
	switch (level) {
		case PlanLevel.Free:
			return "Free";
		case PlanLevel.Team:
			return "Team";
		case PlanLevel.Enterprise:
			return "Enterprise";
		default:
			return "Bencher Plus";
	}
};

export const planLevelPrice = (level: undefined | PlanLevel) => {
	switch (level) {
		case PlanLevel.Free:
			return 0.0;
		case PlanLevel.Team:
			return 0.01;
		case PlanLevel.Enterprise:
			return 0.05;
		default:
			return 0.0;
	}
};

export const suggestedMetrics = (usage: undefined | number) =>
	(Math.round((usage ?? 1) / 1_000) + 1) * 12_000;

export const fmtUsd = (usd: undefined | number) => {
	const numberFmd = new Intl.NumberFormat("en-US", {
		style: "currency",
		currency: "USD",
	});
	return numberFmd.format(usd ?? 0);
};

// https://developer.mozilla.org/en-US/docs/Glossary/Base64#the_unicode_problem
export const base64ToBytes = (base64) => {
	const binString = atob(base64);
	return Uint8Array.from(binString, (m) => m.codePointAt(0));
};

export const decodeBase64 = (base64) =>
	new TextDecoder().decode(base64ToBytes(base64));

export const bytesToBase64 = (bytes) => {
	const binString = String.fromCodePoint(...bytes);
	return btoa(binString);
};

export const encodeBase64 = (str) =>
	bytesToBase64(new TextEncoder().encode(str));
