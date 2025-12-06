export interface LoggerOptions {
  level?: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'fatal';
  base?: Record<string, any>;
}

export interface LogFn {
  (msg: string, ...args: any[]): void;
  (obj: object, msg?: string, ...args: any[]): void;
}

export class Logger {
  constructor(options?: LoggerOptions);
  trace: LogFn;
  debug: LogFn;
  info: LogFn;
  warn: LogFn;
  error: LogFn;
  fatal: LogFn;
  child(bindings: Record<string, any>): Logger;
}

export function pino(options?: LoggerOptions): Logger;

export default pino;

export const levels: {
  values: {
    trace: 10;
    debug: 20;
    info: 30;
    warn: 40;
    error: 50;
    fatal: 60;
  };
  labels: {
    10: 'trace';
    20: 'debug';
    30: 'info';
    40: 'warn';
    50: 'error';
    60: 'fatal';
  };
};
