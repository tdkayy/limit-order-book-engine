import { useEffect, useState } from "react";

function App() {
  const [status, setStatus] = useState("Checking API status...");

  useEffect(() => {
    fetch("/api")
      .then((res) => res.text())
      .then((text) => setStatus(text))
      .catch((err) => {
        console.error("Failed to reach API:", err);
        setStatus("API is unreachable");
      });
  }, []);
  
  return (
    <main className="flex items-center justify-center min-h-screen bg-gray-100 text-2xl font-semibold text-gray-800 font-mono">
      <div className="p-6 bg-white shadow-xl rounded-xl border border-gray-200">
        {status}
      </div>
    </main>
  );
}

export default App;
