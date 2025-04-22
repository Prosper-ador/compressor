export function compress(data) {
  if (!Buffer.isBuffer(data)) {
      throw new Error('Input must be a Buffer');
  }
  
  const result = [];
  let count = 1;
  
  for (let i = 0; i < data.length; i++) {
      if (i < data.length - 1 && data[i] === data[i + 1]) {
          count++;
      } else {
          result.push(data[i], count);
          count = 1;
      }
  }
  
  return Buffer.from(result);
}

export function decompress(data) {
  if (!Buffer.isBuffer(data)) {
      throw new Error('Input must be a Buffer');
  }
  
  const result = [];
  
  for (let i = 0; i < data.length; i += 2) {
      const byte = data[i];
      const count = data[i + 1];
      for (let j = 0; j < count; j++) {
          result.push(byte);
      }
  }
  
  return Buffer.from(result);
}
