import './App.css'

import { Bill as BillComponent } from './components/Bill'
import { ErrorSnack } from './components/ErrorSnack'
import { useBills } from './hooks/use-bills'

export default function App() {
  const [{
    error,
    bills
  }] = useBills([])

  return (
    <>
      <div className='App'>
        {bills.length > 0 && <h1>your bills</h1>}
        {bills.map(bill => <BillComponent key={bill.id} bill={bill} />)}
        {error && <ErrorSnack message={error} />}
      </div>
    </>
  )
}
