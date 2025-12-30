package com.mazerion.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.mazerion.ui.viewmodels.CalculatorDetailViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CalculatorDetailScreen(
    calculatorId: String,
    onBack: () -> Unit,
    viewModel: CalculatorDetailViewModel = viewModel(
        factory = CalculatorDetailViewModel.Factory(calculatorId)
    )
) {
    val uiState by viewModel.uiState.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(uiState.calculatorName) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.onPrimaryContainer
                )
            )
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp)
                .verticalScroll(rememberScrollState()),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // Input fields based on calculator type
            when (calculatorId) {
                "abv" -> ABVCalculatorInputs(viewModel)
                "brix_to_sg" -> BrixToSGInputs(viewModel)
                "sg_to_brix" -> SGToBrixInputs(viewModel)
                "dilution" -> DilutionInputs(viewModel)
                else -> GenericCalculatorInputs(viewModel)
            }

            // Calculate button
            Button(
                onClick = { viewModel.calculate() },
                modifier = Modifier.fillMaxWidth(),
                enabled = !uiState.isLoading
            ) {
                if (uiState.isLoading) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(20.dp),
                        color = MaterialTheme.colorScheme.onPrimary
                    )
                } else {
                    Text("Calculate")
                }
            }

            // Results
            uiState.result?.let { result ->
                ResultCard(result)
            }

            // Error
            uiState.error?.let { error ->
                Card(
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.errorContainer
                    )
                ) {
                    Text(
                        text = error,
                        modifier = Modifier.padding(16.dp),
                        color = MaterialTheme.colorScheme.onErrorContainer
                    )
                }
            }
        }
    }
}

@Composable
fun ABVCalculatorInputs(viewModel: CalculatorDetailViewModel) {
    val params by viewModel.params.collectAsState()

    OutlinedTextField(
        value = params["og"] ?: "",
        onValueChange = { viewModel.updateParam("og", it) },
        label = { Text("Original Gravity (OG)") },
        placeholder = { Text("1.080") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )

    OutlinedTextField(
        value = params["fg"] ?: "",
        onValueChange = { viewModel.updateParam("fg", it) },
        label = { Text("Final Gravity (FG)") },
        placeholder = { Text("1.010") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )
}

@Composable
fun BrixToSGInputs(viewModel: CalculatorDetailViewModel) {
    val params by viewModel.params.collectAsState()

    OutlinedTextField(
        value = params["brix"] ?: "",
        onValueChange = { viewModel.updateParam("brix", it) },
        label = { Text("Brix (°Bx)") },
        placeholder = { Text("20.0") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )
}

@Composable
fun SGToBrixInputs(viewModel: CalculatorDetailViewModel) {
    val params by viewModel.params.collectAsState()

    OutlinedTextField(
        value = params["sg"] ?: "",
        onValueChange = { viewModel.updateParam("sg", it) },
        label = { Text("Specific Gravity (SG)") },
        placeholder = { Text("1.080") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )
}

@Composable
fun DilutionInputs(viewModel: CalculatorDetailViewModel) {
    val params by viewModel.params.collectAsState()

    OutlinedTextField(
        value = params["current_volume"] ?: "",
        onValueChange = { viewModel.updateParam("current_volume", it) },
        label = { Text("Current Volume (L)") },
        placeholder = { Text("10.0") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )

    OutlinedTextField(
        value = params["current_abv"] ?: "",
        onValueChange = { viewModel.updateParam("current_abv", it) },
        label = { Text("Current ABV (%)") },
        placeholder = { Text("14.0") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )

    OutlinedTextField(
        value = params["target_abv"] ?: "",
        onValueChange = { viewModel.updateParam("target_abv", it) },
        label = { Text("Target ABV (%)") },
        placeholder = { Text("12.0") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier.fillMaxWidth()
    )
}

@Composable
fun GenericCalculatorInputs(viewModel: CalculatorDetailViewModel) {
    Text(
        text = "This calculator requires custom inputs",
        style = MaterialTheme.typography.bodyMedium,
        color = MaterialTheme.colorScheme.onSurfaceVariant
    )
}

@Composable
fun ResultCard(result: mazerion.CalcResult) {
    Card(
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.primaryContainer
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Text(
                text = "Result",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onPrimaryContainer
            )

            Text(
                text = result.displayText,
                style = MaterialTheme.typography.headlineMedium,
                color = MaterialTheme.colorScheme.onPrimaryContainer
            )

            if (result.warnings.isNotEmpty()) {
                Divider()
                Text(
                    text = "Warnings:",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onPrimaryContainer
                )
                result.warnings.forEach { warning ->
                    Text(
                        text = "• $warning",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }

            if (result.metadata.entries.isNotEmpty()) {
                Divider()
                Text(
                    text = "Details:",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onPrimaryContainer
                )
                result.metadata.entries.forEach { entry ->
                    Text(
                        text = "${entry.key}: ${entry.value}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }
        }
    }
}