'use strict';
const parser = require('fast-xml-parser');

const isSvg = input => {
	if (input === undefined || input === null) {
		return false;
	}

	// TODO: Remove the `.replace` call when using `fast-xml-parser@4` which has fixed the bug.
	input = input.toString().trim().replace(/\n/g, ' ');

	if (input.length === 0) {
		return false;
	}

	// Has to be `!==` as it can also return an object with error info.
	if (parser.validate(input) !== true) {
		return false;
	}

	let jsonObject;
	try {
		jsonObject = parser.parse(input);
	} catch (_) {
		return false;
	}

	if (!jsonObject) {
		return false;
	}

	if (!('svg' in jsonObject)) {
		return false;
	}

	return true;
};

module.exports = isSvg;
// TODO: Remove this for the next major release
module.exports.default = isSvg;
