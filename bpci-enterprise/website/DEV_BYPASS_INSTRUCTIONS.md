# 🔓 DEV MODE BYPASS - Quick Instructions

## How to Skip Login and Access Internal UI

### Step 1: Start the Development Server
```bash
cd /home/umesh/metanode/bpci-enterprise/website/bpci-enterprise-website
npm run dev
```

### Step 2: Open Browser
Open your browser and go to: `http://localhost:3000` (or whatever port your app runs on)

### Step 3: Enable Dev Bypass
1. Open browser console (Press F12 or Cmd+Option+I)
2. Run this command:
```javascript
localStorage.setItem('DEV_BYPASS_AUTH', 'true')
```
3. Refresh the page (F5 or Cmd+R)

### Step 4: You're In!
You should now bypass the login screen and see the internal UI (Dashboard, etc.)

---

## To Disable Dev Bypass (Use Real Login)

Open browser console and run:
```javascript
localStorage.removeItem('DEV_BYPASS_AUTH')
```

Then refresh the page.

---

## What You'll See

After bypassing auth, you'll have access to:
- ✅ Dashboard (internal UI)
- ✅ Blog (with posting capabilities)
- ✅ Wallet features
- ✅ All authenticated routes

---

## Notes

- This bypass is ONLY for development/testing
- It's added at the top of `AuthContainer.tsx`
- Real authentication code is untouched
- Easy to remove for production (just delete the bypass block)
- Console will show: "🔓 DEV MODE: Auth bypass enabled - skipping login"

---

## Troubleshooting

**If bypass doesn't work:**
1. Make sure you're running the latest code
2. Check browser console for the "🔓 DEV MODE" message
3. Try clearing all localStorage: `localStorage.clear()` then set the flag again
4. Hard refresh: Ctrl+Shift+R (or Cmd+Shift+R on Mac)
