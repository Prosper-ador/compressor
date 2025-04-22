#!/usr/bin/env node

import { handleCLI } from './cli.js';

// Execute CLI if called directly
handleCLI(process.argv).catch(err => {
    console.error('Fatal Error:', err.message);
    process.exit(1);
});
