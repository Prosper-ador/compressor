import fs from 'fs/promises';
import path from 'path';

export async function readFileAsBytes(filePath) {
    return await fs.readFile(filePath);
}

export async function writeBytesToFile(filePath, data) {
    await fs.writeFile(filePath, data);
}

export async function listFilesRecursively(dir) {
    let results = [];
    const list = await fs.readdir(dir, { withFileTypes: true });

    for (const entry of list) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
            const nestedFiles = await listFilesRecursively(fullPath);
            results = results.concat(nestedFiles);
        } else {
            results.push(fullPath);
        }
    }

    return results;
}
