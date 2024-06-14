import * as ChartJS from './chart.js';

(async function() {

    let payments = await fetch('/api/payments');
    payments = await payments.json();
    payments.sort((a, b) => new Date(a.created_at) - new Date(b.created_at));
    payments = payments.filter(payment => payment.state === 'Paid');
    let paymentsByDay = payments.reduce((acc, payment) => {
        let date = new Date(payment.created_at);
        let key = `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;

        if (acc[key]) {
            acc[key] += parseFloat(payment.amount);
        } else {
            acc[key] = parseFloat(payment.amount);
        }

        return acc
    }, {});

    let labels = Object.keys(paymentsByDay);
    let data = Object.values(paymentsByDay);

    console.log(labels, data);
    console.log(paymentsByDay);

    new Chart(
        document.getElementById('payments-chart').getContext('2d'),
        {
            type: 'line',
            data: {
                labels: labels,
                datasets: [
                    {
                        label: 'Payments amount per day',
                        data: data,
                        borderColor: 'rgb(23,64,64)',
                        backgroundColor: 'rgba(146,158,158,0.2)',
                    },
                ],
            },
            options: {
                scales: {
                    y: {
                        beginAtZero: true,
                    },
                },
            },
        }
    );
})();
