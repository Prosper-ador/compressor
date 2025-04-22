const WINDOW_SIZE = 20;

export function compress(data) {
    if (!Buffer.isBuffer(data)) {
        throw new Error('Input must be a Buffer');
    }
    
    const result = [];
    let i = 0;
    
    while (i < data.length) {
        let bestMatch = { offset: 0, length: 0 };
        const windowStart = Math.max(0, i - WINDOW_SIZE);
        
        // Search for matches in the sliding window
        for (let j = windowStart; j < i; j++) {
            let length = 0;
            while (i + length < data.length && 
                   data[j + length] === data[i + length] && 
                   length < 255) {
                length++;
            }
            
            if (length > bestMatch.length) {
                bestMatch = { offset: i - j, length };
            }
        }
        
        if (bestMatch.length > 2) {
            // Encode match
            result.push(0x01, bestMatch.offset, bestMatch.length);
            i += bestMatch.length;
        } else {
            // Encode literal
            result.push(0x00, data[i]);
            i++;
        }
    }
    
    return Buffer.from(result);
}

export function decompress(data) {
    if (!Buffer.isBuffer(data)) {
        throw new Error('Input must be a Buffer');
    }
    
    const result = [];
    let i = 0;
    
    while (i < data.length) {
        const type = data[i];
        
        if (type === 0x00) {
            // Literal
            result.push(data[i + 1]);
            i += 2;
        } else if (type === 0x01) {
            // Match
            const offset = data[i + 1];
            const length = data[i + 2];
            
            for (let j = 0; j < length; j++) {
                result.push(result[result.length - offset]);
            }
            
            i += 3;
        }
    }
    
    return Buffer.from(result);
}
