import { useState } from 'react';
import { useAuth } from '../contexts/AuthContext';

export const LoginScreen = () => {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const { login } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    try {
      const success = await login(password);
      if (!success) {
        console.log('Login failed: incorrect password');
        setError('Senha incorreta');
        setPassword('');
      }
    } catch (err) {
      console.error('Error during login:', err);
      setError('Senha incorreta');
      setPassword('');
    }
  };

  return (
    <div className="min-h-screen bg-dark-bg flex items-center justify-center p-4">
      <div className="bg-dark-surface p-8 rounded-lg border border-dark-border w-full max-w-md">
        <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Gestor do Clube</h1>
        <h2 className="text-xl mb-4 text-dark-text-primary">Login</h2>
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Senha</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full bg-dark-surface border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
              autoFocus
            />
          </div>
          {error && <p className="text-dark-error mb-4 text-sm">{error}</p>}
          <button type="submit" className="w-full bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90">
            Entrar
          </button>
        </form>
      </div>
    </div>
  );
};
