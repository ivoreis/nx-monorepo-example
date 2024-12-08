import './UI.css';
import { useForm } from 'react-hook-form';

export type Platform = 'admin' | 'customer' | 'business';

export const supportedPlatforms: Platform[] = ['admin', 'customer', 'business'];

export type FormData = {
  date: string; // ISO date format
  reason: string;
};

export function Form(props: { apiURL: string }) {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormData>();
  const { apiURL } = props;

  const onSubmit = (baseURL: string) => async (data: FormData) => {
    try {
      const response = await fetch(`${baseURL}/perform`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      });

      if (!response.ok) {
        throw new Error('Failed to submit');
      }

      const result = await response.json();
      alert('Form submitted successfully: ' + JSON.stringify(result));
    } catch (error) {
      console.error('Error submitting form:', error);
      alert('Error submitting form');
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit(apiURL))} className="form">
      <div className="form-group">
        <label htmlFor="date">Date</label>
        <input
          id="date"
          type="date"
          {...register('date', { required: 'Date is required' })}
        />
        {errors.date && <p className="error">{errors.date.message}</p>}
      </div>

      <div className="form-group">
        <label htmlFor="reason">Reason</label>
        <textarea
          id="reason"
          {...register('reason', { required: 'Reason is required' })}
        />
        {errors.reason && <p className="error">{errors.reason.message}</p>}
      </div>

      <button type="submit">Submit</button>
    </form>
  );
}

export default function UI(props: {
  platform: Platform;
  id: string;
  apiURL: string;
}) {
  const { platform, id, apiURL } = props;
  return (
    <div className="ui-wrapper">
      <h2>Remote Cancellation Page</h2>
      <div>
        <p>Platform: {platform}</p>
        <p>ID: {id}</p>
      </div>
      <Form apiURL={apiURL} />
    </div>
  );
}
