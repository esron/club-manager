interface DateInputProps {
  value: string; // YYYY-MM-DD format
  onChange: (value: string) => void; // YYYY-MM-DD format
  className?: string;
  required?: boolean;
}

export const DateInput = ({ value, onChange, className, required }: DateInputProps) => {
  // Convert YYYY-MM-DD to DD/MM/YYYY for display
  const formatToDisplay = (isoDate: string): string => {
    if (!isoDate) return '';
    const [year, month, day] = isoDate.split('-');
    return `${day}/${month}/${year}`;
  };

  // Convert DD/MM/YYYY to YYYY-MM-DD for storage
  const formatToISO = (displayDate: string): string => {
    const cleaned = displayDate.replace(/\D/g, '');
    if (cleaned.length !== 8) return '';

    const day = cleaned.substring(0, 2);
    const month = cleaned.substring(2, 4);
    const year = cleaned.substring(4, 8);

    // Basic validation
    const dayNum = parseInt(day, 10);
    const monthNum = parseInt(month, 10);
    const yearNum = parseInt(year, 10);

    if (dayNum < 1 || dayNum > 31 || monthNum < 1 || monthNum > 12 || yearNum < 1900) {
      return '';
    }

    return `${year}-${month}-${day}`;
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let input = e.target.value;

    // Remove all non-digits
    const digitsOnly = input.replace(/\D/g, '');

    // Auto-format as user types: DD/MM/YYYY
    let formatted = '';
    if (digitsOnly.length > 0) {
      formatted = digitsOnly.substring(0, 2);
      if (digitsOnly.length >= 3) {
        formatted += '/' + digitsOnly.substring(2, 4);
      }
      if (digitsOnly.length >= 5) {
        formatted += '/' + digitsOnly.substring(4, 8);
      }
    }

    // Update display
    e.target.value = formatted;

    // If complete date, convert and send
    if (digitsOnly.length === 8) {
      const isoDate = formatToISO(formatted);
      if (isoDate) {
        onChange(isoDate);
      }
    } else if (digitsOnly.length === 0) {
      onChange('');
    }
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    const input = e.target.value;
    if (input) {
      const isoDate = formatToISO(input);
      if (isoDate) {
        onChange(isoDate);
        e.target.value = formatToDisplay(isoDate);
      }
    }
  };

  return (
    <input
      type="text"
      defaultValue={formatToDisplay(value)}
      onChange={handleChange}
      onBlur={handleBlur}
      placeholder="DD/MM/AAAA"
      className={className}
      required={required}
      maxLength={10}
    />
  );
};
