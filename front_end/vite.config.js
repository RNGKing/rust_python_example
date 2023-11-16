import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  build:{
    outDir:'E:\\rust_demo_projects\\rust_python_example\\back_end\\app'
  },
  plugins: [react()],
})
