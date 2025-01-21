import { clsx, type ClassValue } from 'clsx';
import { QueryExecResult } from 'sql.js';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function removeAllNewLines(input: string) {
  return input.replace(/\n/g, '');
}

export function convertMapToObject(map: Map<string, string>) {
  return Object.fromEntries(map);
}

export function convertQueryResultToObjects(result: QueryExecResult) {
  const { columns, values } = result;
  return values.map((row) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const obj: { [key: string]: any } = {};
    columns.forEach((col, index) => {
      obj[col] = row[index];
    });
    return obj;
  });
}
