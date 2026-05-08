export const HelpScreen = () => {
  return (
    <div className="flex-1 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-2xl font-bold mb-8 text-dark-text-primary">Ajuda</h1>

        {/* About Section */}
        <div className="bg-dark-surface p-6 rounded-lg border border-dark-border mb-6">
          <h2 className="text-xl font-semibold mb-4 text-dark-text-primary">Gestor do Clube</h2>
          <p className="text-dark-text-secondary mb-2">Versão 1.0.0</p>
          <p className="text-dark-text-secondary mb-4">
            Aplicativo para gestão de mensalidades do clube.
          </p>
          <p className="text-dark-text-secondary text-sm">
            Desenvolvido com Tauri + React
          </p>
        </div>

        {/* Quick Start Guide */}
        <div className="bg-dark-surface p-6 rounded-lg border border-dark-border mb-6">
          <h2 className="text-xl font-semibold mb-4 text-dark-text-primary">Guia Rápido</h2>

          <div className="space-y-4">
            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Adicionar membro</h3>
              <p className="text-dark-text-secondary">
                Clique em "Membros" → "Adicionar Membro". Informe o nome e a data de início da participação no clube.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Registrar pagamento</h3>
              <p className="text-dark-text-secondary">
                Use o botão "Adicionar Pagamento" no topo da tela. Selecione o membro, o mês de referência, e a data do pagamento.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Visualizar dívidas</h3>
              <p className="text-dark-text-secondary">
                O Dashboard mostra a dívida total do clube. Para ver dívidas por membro, acesse a aba "Membros" e clique no nome do membro.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Cálculo de dívidas</h3>
              <p className="text-dark-text-secondary">
                Um mês sem pagamento se torna dívida após o dia 10 do mês seguinte. Exemplo: sem pagamento em março → dívida após 10 de abril.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Exportar relatórios</h3>
              <p className="text-dark-text-secondary">
                Acesse a aba "Relatórios". Escolha o tipo de relatório (dívidas ou histórico de pagamentos), configure as opções, e clique em "Exportar".
              </p>
            </div>
          </div>
        </div>

        {/* Security Section */}
        <div className="bg-dark-surface p-6 rounded-lg border border-dark-border">
          <h2 className="text-xl font-semibold mb-4 text-dark-text-primary">Segurança e Senha</h2>

          <p className="text-dark-text-secondary mb-4">
            Este aplicativo protege seus dados com criptografia. Sua senha é necessária para acessar o banco de dados.
          </p>

          <div className="bg-yellow-900/20 border border-yellow-700 p-4 rounded">
            <p className="text-yellow-500 font-semibold mb-2">⚠️ Importante</p>
            <p className="text-yellow-200">
              Não há recuperação de senha. Guarde sua senha em local seguro.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};
