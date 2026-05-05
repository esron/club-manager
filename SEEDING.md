# Database Seeding for Development

The application includes a seeding command for development and testing purposes.

## How to Use

**Note:** The seeding command is only available in development builds (when running `npm run tauri dev`). It will not be available in production builds.

### Via Dev Tools Panel (Easiest)

1. Start the application in development mode:
   ```bash
   npm run tauri dev
   ```

2. Log in to the application

3. Look for the yellow "🛠️ Dev Tools" panel in the bottom-right corner

4. Click the "Seed Database" button

5. Wait for completion (you'll see the result in the panel)

6. Refresh the page (F5) to see the seeded data

### Via Browser Console (Advanced)

1. Start the application in development mode:
   ```bash
   npm run tauri dev
   ```

2. Open the browser console (F12 → Console tab)

3. Run the seeding command:
   ```javascript
   import('@tauri-apps/api/core').then(({ invoke }) => {
     invoke('seed_database', { 
       password: 'your-password',
       memberCount: 100,
       paymentsPerMember: 5
     }).then(result => console.log(result));
   });
   ```

   Replace `'your-password'` with your actual application password.

4. Wait for the console to show "Seed complete!" message

5. Refresh the page (F5) to see the seeded data

### What Gets Created

The seeding command will:
- Create the specified number of members (default: 100)
  - Random Brazilian Portuguese names
  - Random start dates between 2020 and 2025
  - Unique names (numbered to avoid duplicates)

- For each member, create the specified number of payments (default: 5)
  - Random months/years between 2024-2026
  - Random amounts between R$ 10.00 and R$ 25.00
  - Skips duplicates (same member + month + year)

### Example Output

```
Seed complete!
Members created: 100
Payments created: 487
```

(Some payments may be skipped due to duplicate month/year combinations)

### Clean Up

To remove seeded data and start fresh:
1. Close the application
2. Delete the database file: `~/.local/share/GestorDoClube/club.db`
3. Restart the application and set up a new password
