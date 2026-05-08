// Check if running in development mode
export const isDevelopment = import.meta.env.DEV;

// App version (synced with package.json)
export const APP_VERSION = '1.0.0';

// Feature flags
export const FEATURES = {
  devTools: isDevelopment,
  debugLogging: isDevelopment,
};
