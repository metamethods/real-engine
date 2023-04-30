import { appWindow } from '@tauri-apps/api/window'

import styles from './app.module.css';

export default function App() {
  return <main className="p-2">
    <div
      id="topbar"
      className={`${styles.topbar} font-inter relative`}
      data-tauri-drag-region
    >
      <h1 className={`${styles.title} font-bold text-white`}>
        Real Engine <sub> v0.0.1 </sub>
      </h1>

      <div className={styles.right}>
        <button onClick={()=>{appWindow.close()}}>
          <img src="/close.png" />
        </button>
        <button onClick={()=>{appWindow.minimize()}}>
          <img src="/minimize.png"/>
        </button>
      </div>
    </div>

    <div 
      id="applications"
      className={`${styles.applications}`}
    ></div>

    <div
      id="memory"
      className={`${styles.memory}`}
    ></div>
  </main>
}