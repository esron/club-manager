import { useState } from 'react';
import { useAuth } from '../contexts/AuthContext';

export const CreatePasswordScreen = () => {
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [error, setError] = useState('');
  const { setupPassword } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (password.length < 8) {
      setError('Senha deve ter no mínimo 8 caracteres');
      return;
    }

    if (password !== confirmPassword) {
      setError('Senhas não coincidem');
      return;
    }

    try {
      await setupPassword(password);
    } catch (err) {
      console.error('Error creating password:', err);
      setError('Erro ao criar senha');
    }
  };

  return (
    <div className="min-h-screen bg-dark-bg flex items-center justify-center p-4">
      <div className="bg-dark-surface p-8 rounded-lg border border-dark-border w-full max-w-md">
        <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Gestor do Clube</h1>
        <h2 className="text-xl mb-4 text-dark-text-primary">Criar Senha</h2>
        <p className="mb-6 text-dark-text-secondary text-sm">
          Esta é a primeira execução. Crie uma senha para proteger seus dados.
        </p>
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Senha</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full bg-dark-surface border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
            />
          </div>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Confirmar Senha</label>
            <input
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              className="w-full bg-dark-surface border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
            />
          </div>
          {error && <p className="text-dark-error mb-4 text-sm">{error}</p>}
          <button type="submit" className="w-full bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90">
            Criar Senha
          </button>
        </form>
      </div>
    </div>
  );
};
